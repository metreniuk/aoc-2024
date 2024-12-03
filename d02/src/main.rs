fn _first() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("../input-1.txt")?;

    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let res = nums.into_iter().fold(0, |mut acc, line| {
        let mut prev_sign = 0;
        let mut safe = true;

        for (i, window) in line.windows(2).enumerate() {
            let x = window[0];
            let y = window[1];
            let current_diff = x - y;
            let sign = if i == 0 { 0 } else { current_diff.signum() };
            if current_diff.abs() > 3 || current_diff == 0 || sign != prev_sign {
                safe = false;
                break;
            }

            prev_sign = current_diff.signum();
        }

        if safe {
            acc += 1;
        }

        acc
    });

    println!("RES {res}");

    Ok(())
}

fn second() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("../input-1.txt")?;

    let nums: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    fn is_safe(line: &mut Vec<i32>) -> bool {
        let mut prev_sign = 0;

        for (i, window) in line.windows(2).enumerate() {
            let x = window[0];
            let y = window[1];

            let current_diff = x - y;
            let sign = if i == 0 { 0 } else { current_diff.signum() };
            if current_diff.abs() > 3 || current_diff == 0 || sign != prev_sign {
                return false;
            }

            prev_sign = current_diff.signum();
        }
        return true;
    }

    let res = nums
        .into_iter()
        .enumerate()
        .fold(0, |mut acc, (i, mut line)| {
            let mut safe = is_safe(&mut line);
            if !safe {
                for (i, _) in line.iter().enumerate() {
                    let mut new_line = line.clone();
                    new_line.remove(i);
                    let new_safe = is_safe(&mut new_line);
                    if new_safe {
                        safe = true;
                        break;
                    }
                }
            }

            if safe {
                acc += 1;
            } else {
                println!("{}: safe {} {:?}", i + 1, safe, line);
            }

            acc
        });

    println!("RES {res}");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    second()?;
    Ok(())
}
