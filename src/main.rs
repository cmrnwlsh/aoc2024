#![feature(iter_map_windows)]

use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env,
    num::ParseIntError,
};

static EXAMPLE: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", d2p1().await?);
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

async fn d1p1() -> Result<u32> {
    let (mut l, mut r) = (
        BinaryHeap::<Reverse<u32>>::new(),
        BinaryHeap::<Reverse<u32>>::new(),
    );
    let mut flag = false;
    for n in input(1).await?.split_whitespace() {
        if flag { &mut l } else { &mut r }.push(Reverse(n.parse()?));
        flag = !flag
    }
    Ok(l.into_sorted_vec()
        .iter()
        .zip(r.into_sorted_vec().iter())
        .fold(0, |acc, (l, r)| acc + l.0.abs_diff(r.0)))
}

async fn d1p2() -> Result<u32> {
    let (mut l, mut r) = (Vec::<u32>::new(), Vec::<u32>::new());
    let mut flag = false;
    for n in input(1).await?.split_whitespace() {
        if flag { &mut l } else { &mut r }.push(n.parse()?);
        flag = !flag
    }
    let f = r.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    Ok(l.iter().fold(0, |acc, n| acc + n * f.get(n).unwrap_or(&0)))
}

async fn d2p1() -> Result<u32> {
    fn cond1(v: &[u32]) -> bool {
        v.iter().map_windows(|[x, y]| x <= y).all(|b| b)
    }
    fn cond2(v: &[u32]) -> bool {
        v.iter().map_windows(|[x, y]| x >= y).all(|b| b)
    }
    fn cond3(v: &[u32]) -> bool {
        v.iter()
            .map_windows(|[x, y]| matches!(x.abs_diff(**y), 1..=3))
            .all(|b| b)
    }
    let reports = input(2)
        .await?
        .trim()
        .split('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|l| l.parse().map_err(|e: ParseIntError| e.into()))
                .collect()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;
    Ok(reports
        .iter()
        .map(|r| ((cond1(r) || cond2(r)) && cond3(r)))
        .filter(|b| *b)
        .count() as u32)
}
