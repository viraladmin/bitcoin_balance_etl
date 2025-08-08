# Bitcoin Balance ETL

A high-performance Bitcoin balance ETL tool for extracting wallet balances directly from Bitcoin Core's `chainstate` LevelDB, even on pruned nodes.

It periodically pauses `bitcoind`, decodes the UTXO set, and writes wallet address balances to PostgreSQL.

It also includes a analytics CLI API tool for examining all balance related data.

---

## Show Support

I make nothing creating and sharing the tools I develop. I do it for my love of the space and the people in it.  

Help a fellow dev out, I ain't vibe coding here. Whats a sat or two between friends? :)

Bitcoin: bc1ql9xt4l62ly6pp7s9358rkdefrwc0mm5yne78xl

---

## Requirements

- Rust (edition 2024+)
- PostgreSQL
- `bitcoind` fully synced (pruned or full)
- Linux (recommended)
- `systemd` or supervisor (recommended for `--unsafe` mode)

---

## Installation

```bash
cargo install bitcoin_balance_etl
```

---

## Environment Setup

Create an `.env` file in users home directory  
or in `/etc/bitcoin_balance_etl` if setting up  
as a service

```env
# Your postgres database username, 
# passwords, port and database in 
# URL format:
DATABASE_URL="postgres://<user>:<password>@localhost:5432/wallet_balances"

# Your bitcoin start command
BITCOIN_START_CMD="bitcoind -daemon -conf=/path/to/bitcoin.conf"

# Path to bitcoin node
BITCOIN_PATH="/home/user/.bitcoin"

# How often in hours to update the 
# database (24 recommended)
LOOPTIME="24"
```

---

## Systemd startup script

Create a startup script in /etc/systemd/system/bitcoin_balance_etl.service  
as the root user.

```bash
[Unit]
Description=Bitcoin Balance ETL Service
After=network.target

[Service]
ExecStart=/usr/local/bin/bitcoin_balance_etl --unsafe
Restart=always
RestartSec=3
User=YourUserName
WorkingDirectory=/etc/bitcoin_balance_etl
StandardOutput=append:/var/log/bitcoin_balance_etl.log
StandardError=append:/var/log/bitcoin_balance_etl.err.log

[Install]
WantedBy=multi-user.target
```

After creating the startup script, run:

```bash
sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl start bitcoin_balance_etl
```

To verify it's running:

```bash
sudo systemctl status bitcoin_balance_etl
```

### Notes:

If building from source ensure you copy binary files  
`bitcoin_balance_etl` and `balance_api` to your  
`/usr/local/bin/`

---

## Manual ETL Usage

```bash
bitcoin_balance_etl [--safe | --unsafe]
```

### `--unsafe`

- For advanced setups (e.g. `systemd` or `supervisord` with restart-on-crash)
- Minimal `bitcoind` downtime (sub-10s)
- Ideal for production environments that can tolerate rare crashes
- Fast and efficient ETL, only failing in rare edge cases due to shared access timing issues

### `--safe`

- Guarantees stability — no `chainstate` file access conflict
- Accepts longer `bitcoind` downtime (2–5 minutes)
- Ensures no `Bus error`, even under heavy load
- Ideal for cautious operators or systems without automatic recovery

---

## What It Does

- Stops `bitcoind`
- Takes a snapshot of the `chainstate` directory
- Restarts `bitcoind` (immediately in `--unsafe`, after processing in `--safe`)
- Iterates through UTXO database to extract addresses and balances
- Writes results to PostgreSQL in batches
- Repeats hourly

---

## Output Table

The tool automatically creates the following table. Database  
must exist first.

```sql
CREATE TABLE wallet_balances (
    wallet_address TEXT,
    balance_sats BIGINT NOT NULL
);
```

---

## Reindex

The tool runs a `REINDEX` on the `wallet_balances` table each  
cycle to keep query performance optimal.

---

## Schedule

Use a systemd service or just let it run persistently.   
It runs an update based on configured time in .env file.

---

## API Usage

```bash
balance_api

  total_bitcoin
  address_count
  address bal <address>
  balance greater_than  <btc>
  balance less_than  <btc>
  balance between <small btc> <large btc>
```  

### `total_bitcoin`

Total bitcoin recorded by ETL

### `address_count`

Total addresses recorded by the ETL

### `address bal <address>`

Total holdings of a given wallet address as reported by the ETL

### ` balance greater_than <btc>`

Count of how many addresses and what eprcentage of addresses have greater than given bitcoin amount as reported by the ETL

### ` balance less_than <btc>`

Count of how many addresses and what percentage of addresses have less than given bitcoin amount as reported by the ETL

### ` balance between <small_btc> <large_btc>`

Count of how many addresses and what percentage of addresses have between two given amounts of bitcoin as reported by the ETL

## License

MIT

##  Disclaimer

These tools are provided as is for educational and research purposes only. No warranty is provided for any damages incured by using these tools.
