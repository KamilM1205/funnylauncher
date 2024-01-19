use crate::utils::constants::URL;

pub fn is_valid_files() -> bool {
    let mut path = dirs::data_dir().unwrap();
    path.push(".funnycraft");

    let checksum = match checksumdir::checksumdir(path.to_str().unwrap()) {
        Ok(s) => s,
        Err(_) => "error".to_string(),
    };

    if checksum == "error" {
        return false;
    }

    let resp = reqwest::blocking::get(format!("{}/checksum", URL)).unwrap().text().unwrap_or_default();
    println!("{} {}", resp, checksum);
    
    if resp != checksum {
        return false;
    }

    true
}