use std::env;
use std::error::Error;
use std::fs;
use std::io::{stdout, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn stop_bitcoind() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Stop bitcoind
    let stop_status = Command::new("bitcoin-cli")
        .arg("stop")
        .status()?;
    if !stop_status.success() {
        return Err("Failed to stop Bitcoin node".into());
    }
    sleep(Duration::from_secs(5)); // Allow graceful shutdown
    Ok(())
}

pub fn start_bitcoind() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Restart bitcoind
    let start_cmd = env::var("BITCOIN_START_CMD")?;
    let restart_status = Command::new("sh")
        .arg("-c")
        .arg(&start_cmd)
        .status()?;
    if !restart_status.success() {
        return Err("Failed to restart Bitcoin node".into());
    }
    Ok(())
}

pub fn prepare_chainstate_snapshot() -> Result<(), Box<dyn Error + Send + Sync>> {
    let bitcoin_path = env::var("BITCOIN_PATH")?;
    let chainstate_path = bitcoin_path.clone() + "/chainstate";

    // Prepare target snapshot path
    let target_path = dirs::home_dir()
        .ok_or("Could not get home directory")?
        .join("chainstate_temp");

    if target_path.exists() {
        fs::remove_dir_all(&target_path)?; // Clean old snapshot
    }

    // Delete LOG and LOG.old (optional WALs)
    let _ = fs::remove_file(format!("{}/LOG", chainstate_path));
    let _ = fs::remove_file(format!("{}/LOG.old", chainstate_path));
    let _ = fs::remove_file(format!("{}/LOCK", chainstate_path));

    // Perform instant hardlink snapshot
    let snapshot_cmd = format!("cp -al '{}' '{}'", chainstate_path, target_path.display());
    let snapshot_status = Command::new("sh")
        .arg("-c")
        .arg(&snapshot_cmd)
        .status()?;
    if !snapshot_status.success() {
        return Err("Failed to create chainstate snapshot with cp -al".into());
    }

    Ok(())
}

pub fn cleanup_chainstate_snapshot() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let target = dirs::home_dir().unwrap().join("chainstate_temp");
    if target.exists() {
        writeln!(stdout(), "🧹 Deleting temporary chainstate at {:?}", target.display()).unwrap();
        stdout().flush().unwrap();
        fs::remove_dir_all(&target)?;
    } else {
        println!("ℹ️ No temp chainstate to clean up.");
    }
    Ok(())
}
