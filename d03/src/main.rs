use regex::Regex;

fn first() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("../input-1.txt")?;
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let re_2 = Regex::new(r"do(n't)?\(\)").unwrap();
    // let mut sum = 0;

    let mut values: Vec<(i64, usize)> = vec![];

    for ((_, [left, right]), end) in re.captures_iter(&input).map(|x| {
        // println!("@@ {:?}", x);
        (x.extract(), x.get(0).unwrap().end())
    }) {
        let left: i64 = left.parse()?;
        let right: i64 = right.parse()?;

        values.push((left * right, end));
        // println!("res {} {}", left, right);
    }
    let mut dos: Vec<(bool, usize)> = vec![(true, 0)];
    for cap in re_2.captures_iter(&input) {
        let m = cap.get(0).unwrap();

        if m.as_str().contains("don't") {
            dos.push((false, m.end()))
        } else {
            dos.push((true, m.end()))
        }
    }

    dos.push((false, 1000000000));

    let mut j = 0;

    let mut sum = 0;

    for chunk in dos.windows(2) {
        let left = chunk[0];
        let right = chunk[1];
        if j >= values.len() {
            continue;
        }
        let mut curr_val = values[j];
        println!("chunks {:?}", left);
        while right.1 > curr_val.1 && j < values.len() {
            if left.0 {
                sum += curr_val.0;
                println!("do {:?}", curr_val.0);
            } else {
                println!("don't {:?} {:?}", curr_val.0, left);
            }
            j += 1;
            if j >= values.len() {
                break;
            }
            curr_val = values[j];
        }
    }

    println!("res {:?} {:?}", sum, dos);
    println!("next {:?}", values);
    println!("sum {}", sum);

    Ok(())
}

fn main() {
    first().unwrap();
}
