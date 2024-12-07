#![feature(iter_map_windows)]
mod d1;
mod d2;
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", d2::p2(&input(2).await?)?);
    Ok(())
}

async fn input(n: u32) -> Result<String> {
    let client = reqwest::Client::new();
    client
        .get(format!("https://adventofcode.com/2024/day/{}/input", n))
        .header("Cookie", env::var("SESSION")?)
        .send()
        .await?
        .text()
        .await
        .map_err(|e| e.into())
}
