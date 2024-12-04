type Matrix = Vec<Vec<char>>;

fn dfs(arr: &Matrix, target_str: &str) -> usize {
    let directions: Vec<(i64, i64)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;

    for (x, row) in arr.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            let mut chars = target_str.chars();
            let mut next_target = chars.next().unwrap();
            let mut past_chars = vec![];
            if next_target != *char {
                continue;
            }

            for dir in directions.iter() {
                let mut dx = x as i64 - dir.0;
                let mut dy = y as i64 - dir.1;

                loop {
                    let next_char = arr.get(dx as usize).and_then(|r| r.get(dy as usize));

                    if next_char.is_none() {
                        break;
                    }

                    let next_char = next_char.unwrap();

                    if let Some(next_target_inner) = chars.next() {
                        next_target = next_target_inner;
                        dx = dx - dir.0;
                        dy = dy - dir.1;
                    } else {
                        if next_target != *next_char {
                            break;
                        }
                        count += 1;
                        break;
                    }

                    if next_target != *next_char {
                        break;
                    }
                    past_chars.push(next_char);
                }

                println!("PAST {:?}", past_chars);
            }
        }
    }

    count
}

fn check_1(arr: &Matrix, target_str: &str) -> usize {
    let directions: Vec<(i64, i64)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;

    for (x, row) in arr.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char != target_str.chars().next().unwrap() {
                continue;
            }
            for dir in directions.iter() {
                let mut dx = x as i64 - dir.0;
                let mut dy = y as i64 - dir.1;
                let mut path_taken = vec![];
                let mut current_collected = String::from(char.to_string());
                let target_length = target_str.len();

                while current_collected.len() < target_length {
                    let next_char = arr.get(dx as usize).and_then(|r| r.get(dy as usize));
                    if let Some(next) = next_char {
                        current_collected.push_str(&next.to_string());
                        path_taken.push((dx, dy));
                        dx = dx as i64 - dir.0;
                        dy = dy as i64 - dir.1;
                    } else {
                        break;
                    }
                }
                if current_collected == target_str {
                    println!("PAST {} {} {:?} {:?}", x, y, current_collected, path_taken);
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_2(arr: &Matrix, target_str: &str) -> Vec<((i64, i64), (i64, i64))> {
    let directions: Vec<(i64, i64)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut coords = vec![];

    for (x, row) in arr.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            if *char != target_str.chars().next().unwrap() {
                continue;
            }
            for dir in directions.iter() {
                let mut dx = x as i64 - dir.0;
                let mut dy = y as i64 - dir.1;
                let mut path_taken = vec![];
                let mut current_collected = String::from(char.to_string());
                let target_length = target_str.len();

                while current_collected.len() < target_length {
                    let next_char = arr.get(dx as usize).and_then(|r| r.get(dy as usize));
                    if let Some(next) = next_char {
                        current_collected.push_str(&next.to_string());
                        path_taken.push((dx, dy));
                        dx = dx as i64 - dir.0;
                        dy = dy as i64 - dir.1;
                    } else {
                        break;
                    }
                }
                if current_collected == target_str {
                    // println!("PAST {} {} {:?} {:?}", x, y, current_collected, path_taken);

                    coords.push(((x as i64, y as i64), (dx + dir.0, dy + dir.1)));
                }
            }
        }
    }

    coords
}

fn match_2(coords: Vec<((i64, i64), (i64, i64))>) -> usize {
    // ((0, 0) (2, 2))   ((2, 0) (0, 2))
    // ((0, 0) (2, 2))   ((0, 2) (2, 0))
    let mut count = 0;

    let directions: Vec<_> = vec![
        ((2, 0), (-2, 0)),
        ((0, 2), (0, -2)),
        // ((0, 2), (2, 0)),
        // ((2, 0), (0, 2)),
        // ((0, 2), (-2, 0)),
        // ((-2, 0), (0, 2)),
        // ((0, -2), (-2, 0)),
        // ((-2, 0), (0, -2)),
    ];

    for (start, end) in coords.iter() {
        for (dstart, dend) in directions.iter() {
            let new_start = (start.0 + dstart.0, start.1 + dstart.1);
            let new_end = (end.0 + dend.0, end.1 + dend.1);
            println!("new  {:?} {:?}", new_start, new_end);
            let found = coords.iter().find(|(fstart, fend)| {
                fstart.0 == new_start.0
                    && fstart.1 == new_start.1
                    && fend.0 == new_end.0
                    && fend.1 == new_end.1
            });

            if found.is_some() {
                count += 1;
            }
        }
    }

    count
}

fn inner() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("../input-1.txt")?;

    let mut arr: Matrix = vec![];

    for (_, line) in input.lines().enumerate() {
        let mut row = vec![];

        for (_, letter) in line.chars().enumerate() {
            row.push(letter)
        }

        arr.push(row);
    }

    let coords = check_2(&arr, "MAS");
    let count = match_2(coords.clone());
    println!("coords {:?}", coords);
    println!("COUNT {:?}", count);
    Ok(())
}

fn main() {
    inner().unwrap();
}
