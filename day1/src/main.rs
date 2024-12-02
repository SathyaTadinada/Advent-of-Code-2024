fn main() {
    // Part 1:

    let lines = include_str!("input.txt").lines();
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    for line in lines {
        let parts = line.split_once("   ");
        let (first, second) = parts.unwrap();

        let first: i32 = first.parse().unwrap();
        let second: i32 = second.parse().unwrap();

        first_list.push(first);
        second_list.push(second);
    }

    first_list.sort();
    second_list.sort();

    let mut distance = 0;

    for i in 0..first_list.len() {
        let first = first_list[i];
        let second = second_list[i];

        distance += (first - second).abs();
    }

    println!("Part 1: {}", distance);

    // Part 2:

    let mut pt2 = 0;
    for i in &first_list {
        let number = second_list.iter().filter(|x| **x == *i).count();
        pt2 += i * number as i32;
    }

    println!("Part 2: {}", pt2);
}
