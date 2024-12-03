use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    env,
};

static EXAMPLE: &str = "
";

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", d1p2().await?);
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
