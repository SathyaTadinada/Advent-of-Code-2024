fn main() {
    let lines = include_str!("input.txt").lines();

    part1(lines.clone());
    part2(lines);
}

fn part1(lines: std::str::Lines) {
    let mut sum = 0;
    for line in lines {
        let mut parts = line.split(": ");
        let result = parts.next().unwrap().parse::<i64>().unwrap();
        let numbers = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut operators = vec!['+'; numbers.len() - 1];
        let mut found = false;

        while !found {
            let mut test_value: i64 = numbers[0];
            for i in 0..operators.len() {
                if operators[i] == '+' {
                    test_value += numbers[i + 1];
                } else {
                    test_value *= numbers[i + 1];
                }
            }
            if test_value == result {
                sum += result;
                found = true;
            }
            let mut i = (operators.len() - 1) as isize;
            while i >= 0 {
                if operators[i as usize] == '+' {
                    operators[i as usize] = '*';
                    break;
                } else {
                    operators[i as usize] = '+';
                    i -= 1;
                }
            }
            if i < 0 {
                found = true;
            }
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(lines: std::str::Lines) {
    let mut sum = 0;
    for line in lines {
        let mut parts = line.split(": ");
        let result = parts.next().unwrap().parse::<i64>().unwrap();
        let numbers = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut operators = vec!['+'; numbers.len() - 1];
        let mut found = false;

        while !found {
            let mut test_value: i64 = numbers[0];
            for i in 0..operators.len() {
                if operators[i] == '+' {
                    test_value += numbers[i + 1];
                } else if operators[i] == '*' {
                    test_value *= numbers[i + 1];
                } else {
                    test_value = format!("{}{}", test_value, numbers[i + 1])
                        .parse::<i64>()
                        .unwrap();
                }
            }
            if test_value == result {
                sum += result;
                found = true;
            }
            let mut i = (operators.len() - 1) as isize;
            while i >= 0 {
                if operators[i as usize] == '+' {
                    operators[i as usize] = '*';
                    break;
                } else if operators[i as usize] == '*' {
                    operators[i as usize] = "||".chars().next().unwrap();
                    break;
                } else {
                    operators[i as usize] = '+';
                    i -= 1;
                }
            }
            if i < 0 {
                found = true;
            }
        }
    }

    println!("Part 2: {}", sum);
}
