use std::{error::Error, fs::File, sync::mpsc::Sender};

use log::{debug, error, info};
use reqwest::header::{HeaderValue, RANGE};
use unzpack::Unzpack;

use crate::{launcher::commands::Command, utils::constants::URL};

pub const DOWNLOAD: &str = "MINECRAFT/DOWNLOAD";

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

pub fn download_minecraft(data_sender: Sender<Command>) -> Result<(), Box<dyn std::error::Error>> {
    info!(target: DOWNLOAD, "Starting download minecraft");

    const CHUNK_SIZE: u32 = 1000 * 1024;
    let url = format!("{}/get_minecraft", URL);

    let client = reqwest::blocking::Client::new();
    let res = match client.get(&url).send() {
        Ok(r) => Ok(r),
        Err(e) => {
            error!(target: DOWNLOAD, "Error while sending request to get download. Error: {e}");
            Err(e)
        }
    }?;
    let size = match res.content_length() {
        Some(s) => Some(s),
        None => {
            error!(target: DOWNLOAD, "Couldn't get content length.");
            None
        }
    }
    .ok_or("Couldn't get content length.")?;
    debug!(target: DOWNLOAD, "Content length: {size}");

    let mut path = match dirs::download_dir() {
        Some(d) => Some(d),
        None => {
            error!(target: DOWNLOAD, "Couldn't get download dir.");
            None
        }
    }
    .ok_or("Couldn't get download dir.")?;
    path.push("funnycraft.zip");

    let mut file = match File::create(&path) {
        Ok(f) => Ok(f),
        Err(e) => {
            error!(target: DOWNLOAD, "Couldn't create download file. Error: {e}");
            Err(e)
        }
    }?;

    let p_iter = match PartialRangeIter::new(0, size - 1, CHUNK_SIZE) {
        Ok(p) => Ok(p),
        Err(e) => {
            error!(target: DOWNLOAD, "Error in PartialRangeIter initialize: {e}");
            Err(e)
        }
    }?;
    for (range, downloaded) in p_iter {
        match data_sender.send(Command::DOWNLOAD((downloaded, size))) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(target: DOWNLOAD, "Error while sending \"DOWNLOAD\" command to control thread.");
                Err(e)
            }
        }?;
        let mut response = match client.get(&url).header(RANGE, range).send() {
            Ok(r) => Ok(r),
            Err(e) => {
                error!(target: DOWNLOAD, "Error while sending download data request.");
                Err(e)
            }
        }?;

        match std::io::copy(&mut response, &mut file) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(target: DOWNLOAD, "Error while writing downloaded data to file.");
                Err(e)
            }
        }?;
    }

    info!(target: DOWNLOAD, "Download completed.");

    drop(file);

    info!(target: DOWNLOAD, "Starting unziping game...");

    let outpath = match dirs::data_dir() {
        Some(d) => Some(d),
        None => {
            error!(target: DOWNLOAD, "Couldn't get data dir.");
            None
        }
    }
    .ok_or("Couldn't get data dir.")?;

    match data_sender.send(Command::UNZIPING) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(target: DOWNLOAD, "Error while sending \"UNZIPING\" command to control thread.");
            Err(e)
        }
    }?;
    match Unzpack::extract(&path, &outpath) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(target: DOWNLOAD, "Error while unziping game. Error: {e}");
            Err(e)
        }
    }?;

    info!(target: DOWNLOAD, "Game was unzipped.");

    match data_sender.send(Command::PLAY) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(target: DOWNLOAD, "Error while sending \"PLAY\" command to control thread.");
            Err(e)
        }
    }?;
    Ok(())
}
