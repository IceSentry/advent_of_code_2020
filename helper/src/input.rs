use anyhow::{anyhow, Context, Result};
use std::{fs, path::Path};

fn filename(year: u16, day: u8) -> String {
    format!("./inputs/{}/{:02}.txt", year, day)
}

pub fn get_input(year: u16, day: u8) -> Result<String> {
    let filename = filename(year, day);
    let file_path = Path::new(&filename);
    if !file_path.exists() {
        download_input(&file_path, year, day)?;
    }
    fs::read_to_string(&filename).context("Failed to read input file")
}

pub fn download_input(file_path: &Path, year: u16, day: u8) -> Result<()> {
    println!("downloading inputs...");

    let path = &format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let session = std::env::var("COOKIE_SESSION")?;
    let resp = ureq::get(path)
        .set("COOKIE", &format!("session={}", session))
        .call();

    println!("inputs downloaded");

    if resp.ok() {
        fs::create_dir_all(&format!("./inputs/{}/", year))?;
        fs::write(file_path, resp.into_string()?)?;

        println!("file written succesfully");

        Ok(())
    } else {
        Err(anyhow!("{} Failed to download inputs", resp.status()))
    }
}
