use tokio_postgres::{Client, Connection, Socket, NoTls};
use tokio_postgres::tls::NoTlsStream;
use std::error::Error;

struct BalanceResult {
    pub sats: i64,
    pub btc: f64,
}

async fn get_pg_client() -> Result<(Client, Connection<Socket, NoTlsStream>), Box<dyn Error + Send + Sync>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=viraladmin password=9MbZ5eVrhJ dbname=btc_wallets",
        NoTls,
    ).await?;

    Ok((client, connection))
}

async fn get_total_bitcoin(client: &Client) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    let row = client
        .query_one("SELECT SUM(balance_sats)::float8 / 100000000.0 FROM wallet_balances", &[])
        .await?;

    let actual_count: f64 = row.get(0);
    Ok(actual_count)
}

async fn get_address_count(client: &Client) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
    let row = client
        .query_one("SELECT COUNT(*) FROM wallet_balances", &[])
        .await?;

    let actual_count: i64 = row.get(0);
    Ok(actual_count)
}

async fn get_address_coverage(client: &Client, given_count: i64) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    let row = client
        .query_one("SELECT COUNT(*) FROM wallet_balances", &[])
        .await?;

    let actual_count: i64 = row.get(0);

    if actual_count == 0 {
        return Ok(0.0);
    }

    let percentage = (given_count as f64 / actual_count as f64) * 100.0;
    Ok(percentage)
}

async fn get_address_balance(
    client: &Client,
    address: &str,
) -> Result<BalanceResult, tokio_postgres::Error> {
    let row = client
        .query_opt(
            "SELECT balance_sats FROM wallet_balances WHERE wallet_address = $1",
            &[&address],
        )
        .await?;

    let sats: i64 = match row {
        Some(row) => row.get(0),
        None => 0,
    };

    let btc = sats as f64 / 100_000_000.0;

    Ok(BalanceResult { sats, btc })
}

pub async fn balance_greater_than(client: &Client, btc: f64) -> Result<(i64, f64), Box<dyn std::error::Error + Send + Sync>> {
    let sats = (btc * 100_000_000.0).round() as i64;
    let row = client
        .query_one("SELECT COUNT(*) FROM wallet_balances WHERE balance_sats > $1", &[&sats])
        .await?;
    let count: i64 = row.get(0);
    let coverage: f64 = get_address_coverage(client, count).await?;
    Ok((count, coverage))
}

pub async fn balance_less_than(client: &Client, btc: f64) -> Result<(i64, f64), Box<dyn std::error::Error + Send + Sync>> {
    let sats = (btc * 100_000_000.0).round() as i64;
    let row = client
        .query_one("SELECT COUNT(*) FROM wallet_balances WHERE balance_sats < $1", &[&sats])
        .await?;
    let count: i64 = row.get(0);
    let coverage: f64 = get_address_coverage(client, count).await?;
    Ok((count, coverage))
}

pub async fn balance_between(client: &Client, btc1: f64, btc2: f64) -> Result<(i64, f64), Box<dyn std::error::Error + Send + Sync>> {
    let sats1 = (btc1 * 100_000_000.0).round() as i64;
    let sats2 = (btc2 * 100_000_000.0).round() as i64;
    let (low, high) = if sats1 < sats2 { (sats1, sats2) } else { (sats2, sats1) };
    let row = client
        .query_one("SELECT COUNT(*) FROM wallet_balances WHERE balance_sats > $1 AND balance_sats < $2 ", &[&low, &high])
        .await?;
    let count: i64 = row.get(0);
    let coverage: f64 = get_address_coverage(client, count).await?;
    Ok((count, coverage))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (client, connection) = get_pg_client().await?;
    tokio::spawn(async move {
        let _ = connection.await;
    });

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  total_bitcoin");
        eprintln!("  address count");
        eprintln!("  address bal <address>");
        eprintln!("  balance greater_than  <btc>");
        eprintln!("  balance less_than  <btc>");
        eprintln!("  balance between <small btc> <large btc>");
        return Ok(());
    }

    match args[1].as_str() {
        "total_bitcoin" => {
            let result = get_total_bitcoin(&client).await?;
            println!("Total bitcoin: {}", result);
            return Ok(());
        }
        "address" => {
            match args[2].as_str() {
                "bal" => {
                    let address = args[3].as_str();
                    let result = get_address_balance(&client, address).await?;
                    println!("Balance: {} BTC ({} sats)", result.btc, result.sats);
                    return Ok(());
                }
                "count" => {
                    let result = get_address_count(&client).await?;
                    println!("Total Wallets: {}", result);
                    return Ok(());
                }
                 _ => {
                     eprintln!("Unknown option for address.");
                     eprintln!("Options:");
                     eprintln!("  address bal <address>");
                     eprintln!("  address count");
                     return Ok(());
                 }
            }
        }
        "balance" => {
             match args[2].as_str() {
                 "greater_than" => {
                     let btc: f64 = args[3].parse::<f64>()?;
                     let (count, percentage) = balance_greater_than(&client, btc).await?;
                     println!("Total Addresses with a blance greater than {:.8} BTC: {} ({:.2})%", btc, count, percentage);
                     return Ok(());
                 }
                 "less_than" => {
                     let btc: f64 = args[3].parse::<f64>()?;
                     let (count, percentage) = balance_less_than(&client, btc).await?;
                     println!("Total Addresses with a balance less than {:.8} BTC: {} ({:.2}%)", btc, count, percentage);
                     return Ok(());
                 }
                 "between" => {
                     let btc1: f64 = args[3].parse::<f64>()?;
                     let btc2: f64 = args[4].parse::<f64>()?;
                     let (count, percentage) = balance_between(&client, btc1, btc2).await?;
                     println!(
                       "Total Addresses with a balance between {:.8} BTC and {:.8} BTC: {} ({:.2}%)",
                        btc1, btc2, count, percentage
                     );
                     return Ok(());
                 }
                 _ => {
                     eprintln!("Unknown option for balance.");
                     eprintln!("Options:");
                     eprintln!("  balance greater_than  <btc>");
                     eprintln!("  balance less_than  <btc>");
                     eprintln!("  balance between <small btc> <large btc>");
                     return Ok(());
                 }
             }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            eprintln!("Commands:");
            eprintln!("  total_bitcoin");
            eprintln!("  address bal <address>");
            eprintln!("  address count");
            eprintln!("  balance greater_than  <btc>");
            eprintln!("  balance less_than  <btc>");
            eprintln!("  balance between <small btc> <large btc>");
            return Ok(());
        }
    }
}
