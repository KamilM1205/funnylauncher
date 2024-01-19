use std::{error::Error, sync::mpsc::Sender, fs::{File, rename, self}, env, io};

use reqwest::header::{HeaderValue, RANGE};
use unzpack::Unzpack;

use crate::{launcher::commands::Command, utils::constants::URL};

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
    const CHUNK_SIZE: u32 = 1000*1024;
    let url = format!("{}/get_minecraft", URL);

    let client = reqwest::blocking::Client::new();
    let res = client.get(&url).send()?;
    let size = res.content_length().unwrap();
    println!("{}", size);

    let mut path = dirs::download_dir().unwrap();
    path.push("funnycraft.zip");

    let mut file = File::create(&path)?;

    for (range, downloaded) in PartialRangeIter::new(0, size - 1, CHUNK_SIZE)? {
        data_sender.send(Command::DOWNLOAD((
            downloaded,
            size,
        )))?;
        let mut response = client.get(&url).header(RANGE, range).send()?;

        std::io::copy(&mut response, &mut file)?;
    }

    drop(file);

    let outpath = dirs::data_dir().unwrap();

    data_sender.send(Command::UNZIPING)?;
    Unzpack::extract(&path, &outpath).unwrap();

    data_sender.send(Command::PLAY)?;
    Ok(())
}