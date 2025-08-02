use bytes::Bytes;
use futures::SinkExt;
use tokio_postgres::{Client, Connection, Socket, NoTls};
use tokio_postgres::tls::NoTlsStream;
use std::error::Error;

pub async fn get_pg_client() -> Result<(Client, Connection<Socket, NoTlsStream>), Box<dyn Error + Send + Sync>> {
    let db_url = dotenvy::var("DATABASE_URL")?;
    let (client, connection) = tokio_postgres::connect(
        &db_url,
        NoTls,
    ).await?;

    Ok((client, connection))
}


pub async fn create_wallet_table(client: &Client) -> Result<(), tokio_postgres::Error> {
    client.execute(
        "
        DROP TABLE IF EXISTS wallet_balances;
        ",
        &[],
    ).await?;

    client.execute(
        "
        CREATE TABLE wallet_balances (
            wallet_address TEXT,
            balance_sats BIGINT
        );
        ",
        &[],
    ).await?;

    Ok(())
}

pub async fn reindex(client: &Client) -> Result<(), tokio_postgres::Error> {
    // Add primary key on wallet_address
    client.execute(
        "
        ALTER TABLE wallet_balances
        ADD PRIMARY KEY (wallet_address);
        ",
        &[],
    ).await?;

    // Add index on balance_sats for fast sorting or filtering
    client.execute(
        "
        CREATE INDEX idx_wallet_balance ON wallet_balances(balance_sats);
        ",
        &[],
    ).await?;

    Ok(())
}

pub async fn copy_batch(
    client: &Client,
    batch: &[(String, i64)],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buf = String::with_capacity(batch.len() * 64);
    for (addr, sats) in batch {
        use std::fmt::Write;
        writeln!(buf, "{}\t{}", addr, sats)?; // tab-separated + newline
    }

    // Start COPY stream
    let sink = client
        .copy_in("COPY wallet_balances (wallet_address, balance_sats) FROM STDIN")
        .await?;

    let mut sink = Box::pin(sink);
    sink.send(Bytes::from(buf)).await?;
    sink.close().await?;

    Ok(())
}
