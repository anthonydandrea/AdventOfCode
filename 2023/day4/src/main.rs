use std::cell::RefCell;
use std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let track: RefCell<HashMap<usize, usize>> = RefCell::new(HashMap::new());

    let tot: usize = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(idx, line)| {
            let (first, nums) = line.split_once("|").unwrap();
            let winning_nums: Vec<_> = first.split_once(":").unwrap().1.trim().split(" ").collect();
            let nums: Vec<_> = nums.trim().split_whitespace().collect();

            let tot = nums.iter().fold(0, |acc, x| {
                if winning_nums.contains(x) {
                    return acc + 1;
                }
                acc
            });

            let this_val = track.borrow().get(&idx).unwrap_or(&0).clone() + 1;

            for i in idx + 1..idx + 1 + tot {
                let new_val = track.borrow().get(&i).unwrap_or(&0) + this_val;
                track.borrow_mut().insert(i, new_val);
            }

            this_val
        })
        .sum();

    println!("{tot:?}");
}
