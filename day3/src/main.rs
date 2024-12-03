use regex::Regex;

fn main() {
    let lines = include_str!("input.txt").lines();

    part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: std::str::Lines) {
    let mut sum = 0;
    for line in lines {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        for item in regex.captures_iter(line) {
            let x: i32 = item[1].parse().unwrap();
            let y: i32 = item[2].parse().unwrap();
            sum += x * y;
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(lines: std::str::Lines) {
    let mut sum = 0;
    let mut mul_enabled = true;
    for line in lines {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        for item in regex.captures_iter(line) {
            if item.get(0).unwrap().as_str() == "do()" {
                mul_enabled = true;
            } else if item.get(0).unwrap().as_str() == "don't()" {
                mul_enabled = false;
            } else if mul_enabled {
                let x: i32 = item[1].parse().unwrap();
                let y: i32 = item[2].parse().unwrap();
                sum += x * y;
            }
        }
    }
    println!("Part 2: {}", sum);
}
