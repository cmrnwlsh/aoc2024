#![feature(iter_map_windows)]

mod d1;
mod d2;
mod d3;
use anyhow::{bail, Result};
use clap::Parser;
use seq_macro::seq;
use std::env;

type Solution = (&'static dyn Fn(&str) -> Result<String>, &'static str);

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,

    #[arg(short, long)]
    example: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let (solution, example) = pmap(args.day, args.part)?;
    let out = if args.example {
        example
    } else {
        &input(args.day).await?
    };
    println!("{}", solution(out)?);
    Ok(())
}

fn pmap(day: u8, part: u8) -> Result<Solution> {
    seq!(N in 1..=3 {
        Ok(match (day, part) {
            #(
                (N, 1) => (&d~N::p1, d~N::EXAMPLE),
                (N, 2) => (&d~N::p2, d~N::EXAMPLE),
            )*
            _ => bail!("invalid selection")
        })
    })
}

async fn input(n: u8) -> Result<String> {
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
