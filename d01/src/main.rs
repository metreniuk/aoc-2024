use std::{collections::HashMap, fs};

fn _first() -> anyhow::Result<()> {
    let input = fs::read_to_string("../input-1.txt")?;

    let (mut left, mut right) = input.lines().fold((vec![], vec![]), |mut acc, line| {
        let (l, r) = line.split_once("   ").unwrap();
        acc.0.push(l.parse::<i64>().unwrap());
        acc.1.push(r.parse::<i64>().unwrap());
        acc
    });

    left.sort();
    right.sort();

    let diff = left
        .into_iter()
        .zip(right)
        .fold(0, |acc, (x, y)| acc + (x - y).abs());

    println!("input {:?} ", diff);

    Ok(())
}

fn second() -> anyhow::Result<()> {
    let input = fs::read_to_string("../input-1.txt")?;

    let (left, right) = input.lines().fold((vec![], vec![]), |mut acc, line| {
        let (l, r) = line.split_once("   ").unwrap();
        acc.0.push(l.parse::<i64>().unwrap());
        acc.1.push(r.parse::<i64>().unwrap());
        acc
    });

    let right_map: HashMap<i64, i64> = right.into_iter().fold(HashMap::new(), |mut acc, x| {
        let entry = acc.entry(x).or_insert(0);
        *entry += 1;

        acc
    });

    let diff = left.into_iter().fold(0, |acc, x| {
        let sim_mult = right_map.get(&x).unwrap_or(&0_i64);
        let sim_score = x * sim_mult;
        acc + sim_score
    });

    println!("input {:?} ", diff);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    second()?;

    Ok(())
}
