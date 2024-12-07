use anyhow::Result;
use std::{collections::HashMap, num::ParseIntError};

pub static EXAMPLE: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn check_windows<T: FnMut(&[&u32; 2]) -> bool>(v: &[u32], e: T) -> bool {
    v.iter().map_windows(e).all(|b| b)
}

fn check(v: &[u32]) -> Option<()> {
    ((check_windows(v, |[x, y]| x <= y) || check_windows(v, |[x, y]| x >= y))
        && check_windows(v, |[x, y]| matches!(x.abs_diff(**y), 1..=3)))
    .then_some(())
}

fn parse_reports(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .trim()
        .split('\n')
        .map(|s| {
            s.split_whitespace()
                .map(|l| l.parse().map_err(|e: ParseIntError| e.into()))
                .collect()
        })
        .collect()
}

pub fn p1(input: &str) -> Result<String> {
    Ok(format!(
        "{}",
        parse_reports(input)?
            .iter()
            .filter_map(|r| check(r))
            .count()
    ))
}
pub fn p2(input: &str) -> Result<String> {
    Ok(format!(
        "{}",
        parse_reports(input)?
            .iter()
            .fold(HashMap::<&Vec<u32>, Vec<Vec<u32>>>::new(), |mut map, v| {
                (0..v.len()).for_each(|i| {
                    let mut tmp = v.clone();
                    tmp.remove(i);
                    map.entry(v).or_insert(vec![tmp.clone()]).push(tmp);
                });
                map
            })
            .iter()
            .filter_map(|(k, v)| {
                check(k).or_else(|| {
                    v.iter()
                        .map(|r| check(r))
                        .any(|e| e.is_some())
                        .then_some(())
                })
            })
            .count()
    ))
}
