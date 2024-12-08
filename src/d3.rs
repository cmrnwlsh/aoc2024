use anyhow::{bail, Result};
use regex::Regex;

pub const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

pub fn p1(input: &str) -> Result<String> {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)")?;
    Ok(format!(
        "{:?}",
        regex.captures_iter(input).try_fold(0, |mut acc, caps| {
            if let Some(m) = caps.get(0) {
                let m = m.as_str();
                let [x, y] = m
                    .strip_prefix("mul(")
                    .unwrap_or(m)
                    .strip_suffix(")")
                    .unwrap_or(m)
                    .split(',')
                    .flat_map(str::parse)
                    .collect::<Vec<u32>>()[..]
                else {
                    bail!("incorrect token")
                };
                acc += x * y;
            }
            Ok(acc)
        })?
    ))
}

pub fn p2(input: &str) -> Result<String> {
    todo!();
}
