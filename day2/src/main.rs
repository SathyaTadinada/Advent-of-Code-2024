fn main() {
    let lines = include_str!("input.txt").lines();

    part1(lines.clone());
    part2(lines.clone());
}

fn part1(lines: std::str::Lines) {
    let mut safe = 0;
    for line in lines {
        if is_safe_pt_1(line) {
            safe += 1;
        }
    }

    println!("Part 1: {}", safe);
}

fn part2(lines: std::str::Lines) {
    // given a report, if it isn't safe, try to see if removing any one element makes it safe
    let mut safe = 0;
    for line in lines {
        let report: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut is_safe = false;

        for i in 0..report.len() {
            let mut report_copy = report.clone();
            report_copy.remove(i);

            if is_safe_pt_1(&report_copy.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")) {
                is_safe = true;
                break;
            }
        }

        if is_safe {
            safe += 1;
        }
    }

    println!("Part 2: {}", safe);
}


fn is_safe_pt_1(line: &str) -> bool {
    let report: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // return false if values are not either in strictly increasing or decreasing order
    if report.windows(2).any(|w| w[0] > w[1]) && report.windows(2).any(|w| w[0] < w[1]) {
        return false;
    }

    // check if each value in report is an absolute difference of between 1 and 3 from the next value
    for i in 0..report.len()-1 {
        let abs_diff = (report[i] - report[i + 1]).abs();
        if !(1..=3).contains(&abs_diff) {
            return false;
        }
    }

    true
       
}