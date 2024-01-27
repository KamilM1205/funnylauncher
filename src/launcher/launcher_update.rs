use std::{
    env,
    error::Error,
    fs::{rename, File},
    sync::mpsc::Sender,
};

use log::{debug, error, info};
use reqwest::header::{HeaderValue, RANGE};

use crate::utils::constants::{URL, VERSION};

const UPDATE: &str = "UPDATE";
const DOWNLOAD: &str = "UPDATE/DOWNLOAD";

#[derive(Default, PartialEq)]
pub struct UpdateData {
    pub downloaded: u64,
    pub size: u64,
}

#[derive(PartialEq)]
pub enum Command {
    Data(UpdateData),
    Completed,
}

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self, Box<dyn Error>> {
        if buffer_size == 0 {
            Err("invalid buffer_size, give a value greater than zero.")?;
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = (HeaderValue, u64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some((
                HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1))
                    .expect("string provided by format!"),
                prev_start,
            ))
        }
    }
}

pub fn download_launcher(data_sender: Sender<Command>) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: DOWNLOAD, "Starting update...");

    const CHUNK_SIZE: u32 = 100 * 1024;
    let url = format!("{}/get_launcher", URL);

    let client = reqwest::blocking::Client::new();
    let res = match client.get(&url).send() {
        Ok(r) => Ok(r),
        Err(e) => {
            error!(target: DOWNLOAD, "Error while sending reqwest to get download. Error: {e}");
            Err(e)
        }
    }?;

    let size = match res.content_length() {
        Some(s) => Some(s),
        None => {
            error!(target: DOWNLOAD, "Content length is empty.");
            None
        }
    }
    .ok_or("Content length is empty.")?;

    debug!(target: DOWNLOAD, "Content length: {size}");

    let mut path = match dirs::download_dir() {
        Some(p) => Some(p),
        None => {
            error!(target: DOWNLOAD, "Download dir not found.");
            None
        }
    }
    .ok_or("Download dir not found.")?;

    path.push("funnylauncher.exe");

    let mut file = match File::create(&path) {
        Ok(f) => Ok(f),
        Err(e) => {
            error!(target: DOWNLOAD, "Couldn't create download file. Error: {e}");
            Err(e)
        }
    }?;

    let p_iter = match PartialRangeIter::new(0, size - 1, CHUNK_SIZE) {
        Ok(iter) => Ok(iter),
        Err(e) => {
            error!(target: DOWNLOAD, "Error in PartialRangeIter initizalize: {e}");
            Err(e)
        }
    }?;

    for (range, downloaded) in p_iter {
        match data_sender.send(Command::Data(UpdateData { downloaded, size })) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(target: DOWNLOAD, "Error while sending UpdateData to control thread.");
                Err(e)
            }
        }?;
        let mut response = match client.get(&url).header(RANGE, range).send() {
            Ok(r) => Ok(r),
            Err(e) => {
                error!(target: DOWNLOAD, "Error while sending dowload data request.");
                Err(e)
            }
        }?;

        match std::io::copy(&mut response, &mut file) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(target: DOWNLOAD, "Error while writing download data to file.");
                Err(e)
            }
        }?;
    }

    info!(target: DOWNLOAD, "Download completed.");
    info!(target: DOWNLOAD, "Moving file...");

    let exe_file = match env::current_exe() {
        Ok(f) => Ok(f),
        Err(e) => {
            error!(target: DOWNLOAD, "Couldn't get launcher executable path. Error: {e}");
            Err(e)
        }
    }?;
    match rename(path, &exe_file) {
        Ok(()) => Ok(()),
        Err(e) => {
            error!(target: DOWNLOAD, "Error while replacing executable file.");
            Err(e)
        }
    }?;

    info!(target: DOWNLOAD, "File moved.");

    std::process::Command::new(&exe_file)
        .stdout(std::process::Stdio::null()) // It's need to launch launcher as proccess, not
        // subproccess
        .stderr(std::process::Stdio::null());

    match data_sender.send(Command::Completed) {
        Ok(()) => Ok(()),
        Err(e) => {
            error!(target: DOWNLOAD, "Error while sending \"Complete\" command to control thread.");
            Err(e)
        }
    }?;

    Ok(())
}

pub fn need_update() -> Result<bool, Box<dyn std::error::Error>> {
    info!(target: UPDATE, "Checking for launcher update...");
    let resp = match reqwest::blocking::get(format!("{}/version", URL)) {
        Ok(r) => Ok(r),
        Err(e) => {
            error!(target: UPDATE, "Error while \"version\" request: {e}");
            msgbox::create(
                "Error",
                &format!("Couldn't connect to update server. Check your internet connection. Error: {e}"),
                msgbox::IconType::Error,
            )
            .unwrap_or_else(|e| {
                error!("Couldn't show msgbox. Error: {e}");
            });
            Err("Couldn't connect to update server.")
        }
    }?;
    let value = match resp.text() {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(target: UPDATE, "Error while trying get content from request.");
            Err(e)
        }
    }?;

    if value != VERSION {
        info!(target: UPDATE, "Launcher need update from {} to {}", VERSION, value);
        return Ok(true);
    }

    info!(target: UPDATE, "Launcher no need update.");
    Ok(false)
}
