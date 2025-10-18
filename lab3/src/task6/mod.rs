use reqwest::blocking;
use std::fs;
use std::io;
use std::io::Write;

pub fn get_file_content(url: &String) -> Result<Vec<u8>, String> {
    let response = match blocking::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            return Err(format!(
                "Failed to get response from {} with error: {}",
                &url,
                err.to_string()
            ));
        }
    };

    if !response.status().is_success() {
        return Err(format!(
            "Failed to get file from url {} with status code {}",
            &url,
            response.status()
        ));
    }
    let bytes: Vec<u8> = match response.bytes() {
        Ok(b) => b.to_vec(),
        Err(err) => return Err(format!("{}", err)),
    };

    return Ok(bytes);
}

pub fn create_file(content: &Vec<u8>) -> Result<(), String> {
    let mut writer = match fs::File::create("response.txt") {
        Ok(f) => io::BufWriter::new(f),
        Err(err) => return Err(format!("{}", err)),
    };

    match writer.write_all(content) {
        Ok(_) => (),
        Err(err) => return Err(format!("{}", err)),
    };

    Ok(())
}
