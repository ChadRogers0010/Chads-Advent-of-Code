use crate::utils::read_file;

pub fn day_one_pt_one_2022() -> i64 {
    let input = read_file("../inputs/2022/day_01.txt").unwrap();
    input
        .join("")
        .split("\n\n")
        .map(|calorie_groups| {
            calorie_groups
                .split("\n")
                .fold(0, |acc, rhs| acc + rhs.parse::<i64>().unwrap())
        })
        .max()
        .unwrap()
}
