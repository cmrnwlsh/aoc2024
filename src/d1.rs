use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub fn d1p1(input: String) -> Result<u32> {
    let (mut l, mut r) = (
        BinaryHeap::<Reverse<u32>>::new(),
        BinaryHeap::<Reverse<u32>>::new(),
    );
    let mut flag = false;
    for n in input.split_whitespace() {
        if flag { &mut l } else { &mut r }.push(Reverse(n.parse()?));
        flag = !flag
    }
    Ok(l.into_sorted_vec()
        .iter()
        .zip(r.into_sorted_vec().iter())
        .fold(0, |acc, (l, r)| acc + l.0.abs_diff(r.0)))
}

pub fn d1p2(input: String) -> Result<u32> {
    let (mut l, mut r) = (Vec::<u32>::new(), Vec::<u32>::new());
    let mut flag = false;
    for n in input.split_whitespace() {
        if flag { &mut l } else { &mut r }.push(n.parse()?);
        flag = !flag
    }
    let f = r.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    Ok(l.iter().fold(0, |acc, n| acc + n * f.get(n).unwrap_or(&0)))
}
