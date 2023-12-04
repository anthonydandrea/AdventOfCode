fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input_split = input.split('\n');

    let map = std::collections::HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let numeric_value = |line: &str, rev: bool| -> String {
        let mut idx: i32 = match rev {
            true => line.len() as i32 - 1,
            false => 0,
        };
        while 0 <= idx && idx < line.len() as i32 {
            if line
                .get(idx as usize..idx as usize + 1)
                .unwrap()
                .matches(char::is_numeric)
                .peekable()
                .peek()
                .is_some()
            {
                return line.get(idx as usize..idx as usize + 1).unwrap().to_owned();
            }

            for (k, v) in map.iter() {
                if idx + k.len() as i32 <= line.len() as i32
                    && line.get(idx as usize..idx as usize + k.len()).unwrap() == *k
                {
                    return (*v).to_owned();
                }
            }

            idx += if rev { -1 } else { 1 };
        }

        "".to_owned()
    };

    let val: i32 = input_split
        .filter(|l| !l.is_empty())
        .map(|line| {
            // println!("{line:?}");
            let first = numeric_value(line, false);
            let second = numeric_value(line, true);
            (first + &second).parse::<i32>().unwrap()
        })
        .sum();

    println!("{val:?}");
}
