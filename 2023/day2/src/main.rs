fn main() {
    let limit_map = std::collections::HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Part 1
    // let tot: u32 = input
    //     .split("\n")
    //     .filter(|line| !line.is_empty())
    //     .map(|line| {
    //         let split: Vec<&str> = line.split(":").collect();
    //         let id = split[0]
    //             .split(" ")
    //             .nth(1)
    //             .unwrap()
    //             .parse()
    //             .unwrap();

    //         println!("Id: {id:?}");

    //         if split[1].split(';').any(|show| {
    //             show.split(",").any(|num_color| {
    //                 let tuple: Vec<_> = num_color.split(" ").filter(|s| !s.is_empty()).collect();
    //                 limit_map[tuple[1]] < tuple[0].parse().unwrap()
    //             })
    //         }) {
    //             0
    //         } else {
    //             id
    //         }
    //     })
    //     .sum();

    // Part 2
    let tot: u32 = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut mins = std::collections::HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            let split: Vec<&str> = line.split(":").collect();

            split[1].split(';').for_each(|show| {
                show.split(",").for_each(|num_color| {
                    let tuple: Vec<_> = num_color.split(" ").filter(|s| !s.is_empty()).collect();
                    mins.insert(
                        tuple[1],
                        std::cmp::max(mins[tuple[1]], tuple[0].parse().unwrap()),
                    );
                });
            });
            println!("{mins:?}");
            mins.values().fold(1, |acc, x| acc * x)
        })
        .sum();

    println!("{tot:?}");
}
