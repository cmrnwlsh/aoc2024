use anyhow::{anyhow, Result};
use regex::Regex;

pub const EXAMPLE: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn get_xy(input: &str) -> Result<(u32, u32)> {
    if let [x, y] = input
        .strip_prefix("mul(")
        .unwrap_or(input)
        .strip_suffix(")")
        .unwrap_or(input)
        .split(',')
        .flat_map(str::parse)
        .collect::<Vec<u32>>()[..]
    {
        Ok((x, y))
    } else {
        Err(anyhow!("invalid input"))
    }
}

pub fn p1(input: &str) -> Result<String> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
    Ok(format!(
        "{:?}",
        regex.captures_iter(input).fold(0, |mut acc, caps| {
            if let Some(m) = caps.get(0)
                && let Ok((x, y)) = get_xy(m.as_str())
            {
                acc += x * y;
            }
            acc
        })
    ))
}

pub fn p2(input: &str) -> Result<String> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)")?;
    let mut enabled = true;
    Ok(format!(
        "{:?}",
        regex.captures_iter(input).fold(0, |mut acc, caps| {
            if let Some(m) = caps.get(0) {
                let m = m.as_str();
                match m {
                    "do()" => enabled = true,
                    "don't()" => enabled = false,
                    _ if let Ok((x, y)) = get_xy(m) => {
                        if enabled {
                            acc += x * y;
                        }
                    }
                    _ => (),
                }
            }
            acc
        })
    ))
}
