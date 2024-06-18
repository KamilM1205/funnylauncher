use std::error::Error;

use log::{debug, info};

use crate::utils::constants::{GET_MINECRAFT_CHECKSUM, URL};

pub const VALIDATOR: &str = "MINECRAFT/VALIDATOR";

pub fn is_valid_files() -> Result<bool, Box<dyn Error>> {
    let mut path = dirs::data_dir().ok_or("Couldn't get data dir")?;
    path.push(".funnycraft");

    info!(target: VALIDATOR, "Checking game hash...");

    let checksum =
        checksumdir::checksumdir(path.to_str().ok_or("Couldn't convert PathBuf to str.")?)?;
    let resp = reqwest::blocking::get(format!("{}{}", URL, GET_MINECRAFT_CHECKSUM))?
        .text()
        .unwrap_or_default();
    debug!(target: VALIDATOR, "Local hash: {checksum} | Server hash: {resp}");

    if resp != checksum {
        return Ok(false);
    }

    Ok(true)
}
