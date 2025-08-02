use bitcoin::{Network};
use dotenvy::dotenv;
use leveldb::database::Database;
use leveldb::database::cache::Cache;
use leveldb::iterator::Iterable;
use leveldb::options::{Options, ReadOptions};
use std::collections::HashMap;
use std::path::Path;
use tokio::time::{sleep, Duration};

mod address_parser;

mod btc;
use btc::{ cleanup_chainstate_snapshot, prepare_chainstate_snapshot, start_bitcoind, stop_bitcoind };

mod bytes_key;

mod numbers;

mod utxo_decoder;

mod postgres;
use postgres::{ copy_batch, get_pg_client, reindex };

const BATCH_SIZE: usize = 5_000_000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Startup Flags:");
        eprintln!("--unsafe \nFor advanced setups (e.g. systemd or supervisord with restart-on-crash) \nMinimal bitcoind downtime (sub-10s), ideal for production environments that can tolerate rare crashes. \nFast and efficient ETL, only failing in rare edge cases due to shared access timing issues.");
        eprintln!("\n--safe \nGuarantees stability — no chainstate file access conflict. \nAccepts longer bitcoind downtime (2 - 5 mins), but ensures no Bus error, even with heavy load. \nIdeal for cautious operators or systems without automatic recovery.");
        return Ok(());
    }
    let mut flag = String::new();
    match args[1].as_str() {
        "--unsafe" => {
            flag = "unsafe".to_string();
        }
        "--safe" => {
            flag = "safe".to_string();
        }
        _ => {

            eprintln!("Startup Flags: {}", flag);
            eprintln!("--unsafe \nFor advanced setups (e.g. systemd or supervisord with restart-on-crash).\nMinimal downtime (sub-10s), ideal for production environments that can tolerate rare crashes.\nFast and efficient ETL, only failing in rare edge cases due to shared access timing issues.");
        eprintln!("\n--safe \nGuarantees stability — no chainstate file access conflict. \n Accepts longer bitcoind downtime (2 - 5 mins), but ensures no Bus error, even with heavy load. \nIdeal for cautious operators or systems without automatic recovery.");
            return Ok(());
        }
    }

    dotenv().ok();
    loop {
        let _ = stop_bitcoind();

        let _ = prepare_chainstate_snapshot();

        if flag == "unsafe" {
            let _ = start_bitcoind();
        }

        let chainstate_path = dirs::home_dir()
            .ok_or("Could not get home directory")?
            .join("chainstate_temp");
        let path = Path::new(&chainstate_path);

        let mut options = Options::new();
        options.create_if_missing = false;
        options.cache = Some(Cache::new(16 * 1024 * 1024));


        let db = match Database::<bytes_key::BytesKey>::open(path, options) {
            Ok(db) => db,
            Err(e) => {
                panic!("LevelDB open failed: {}", e);
            }
        };


        let read_opts = ReadOptions::new();
        let xor_iter = db.iter(read_opts);
        let mut obfuscation_key: Vec<u8> = Vec::new();
        for (key, value) in xor_iter {
            if key.0.starts_with(&[0x0e]) {
                obfuscation_key = value[1..].to_vec();
                break;
            }
        }

        let read_opts = ReadOptions::new();
        let iter = db.iter(read_opts);
        let mut wallet_map: HashMap<String, i64> = HashMap::new();

        for (key, value) in iter {
            let decoded_value: Vec<u8> = obfuscation_key
                .iter()
                .cycle()
                .zip(value.iter())
                .map(|(k, v)| k ^ v)
                .collect();

            if let Some((_txid, _vout)) = utxo_decoder::decode_utxo_key(&key.0) {
                let mut value_slice = decoded_value.as_slice();
                if let Some((amount, script)) = utxo_decoder::decode_utxo_value(&mut value_slice) {
                    if amount > 0 {
                        let label = address_parser::parse_address(&script, Network::Bitcoin);
                        *wallet_map.entry(label).or_insert(0) += amount;
                    }
                }
            }
        }
        let wallets: Vec<(String, i64)> = wallet_map.into_iter().collect();
 
        drop(db);
        let _ = cleanup_chainstate_snapshot();
        if flag == "safe" { 
            let _ = start_bitcoind();
        }

        let (client, connection) = get_pg_client().await?;

        let conn_task = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });

        postgres::create_wallet_table(&client).await?;
        drop(client);
        let _ = conn_task.await;

        let mut tasks = Vec::new();

        for chunk in wallets.chunks(BATCH_SIZE) {
            let batch = chunk.to_vec(); // move into task
            tasks.push(tokio::spawn(async move {
                let (client, connection) = get_pg_client().await?;

                let conn_task = tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("PostgreSQL connection error: {}", e);
                    }
                });

                copy_batch(&client, &batch).await?;
                drop(client);
                let _ = conn_task.await;
                Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
            }));
        }

        for task in tasks {
            if let Err(e) = task.await? {
                eprintln!("Task failed: {}", e);
            }
        }

        let (client, connection) = get_pg_client().await?;

        let conn_task = tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });
        drop(wallets);
        reindex(&client).await?;
        drop(client);
        let _ = conn_task.await;

        sleep(Duration::from_secs(60 * 60)).await;
    }
}
