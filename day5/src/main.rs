fn main() {
    let arr = include_str!("input.txt").lines().map(|x| x.to_string()).collect();
    let (rules, pages) = parse(arr);

    println!("{}", part1(&rules, &pages));
    println!("{}", part2(&rules, &pages));
}

fn part1(rules: &[(i32, i32)], pages: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for row in pages {
        if is_ordered(rules, row) {
            let middle_index = row.len() / 2;
            sum += row[middle_index];
        }
    }
    sum
}

fn part2(rules: &[(i32, i32)], pages: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    for row in pages {
        if !is_ordered(rules, row) {
            let new_row = re_order(rules, &mut row.clone());
            let middle_index = new_row.len() / 2;
            sum += new_row[middle_index];
        }
    }
    sum
}

fn parse(arr: Vec<String>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();

    let mut sections = arr.split(|line| line.is_empty());
    let rules_section = sections.next().unwrap();
    let pages_section = sections.next().unwrap();

    for line in rules_section {
        let parts: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
        rules.push((parts[0], parts[1]));
    }

    for line in pages_section {
        let parts: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        pages.push(parts);
    }
    (rules, pages)
}

fn is_ordered(rules: &[(i32, i32)], row: &[i32]) -> bool {
    let mut ordered = true;
    for rule in rules {
        let (x, y) = rule;
        if row.contains(x) && row.contains(y) {
            let x_index = row.iter().position(|&r| r == *x).unwrap();
            let y_index = row.iter().position(|&r| r == *y).unwrap();
            ordered &= x_index < y_index;
        }
    }
    ordered
}

fn re_order(rules: &[(i32, i32)], row: &mut [i32]) -> Vec<i32> {
    let mut ordered = false;
    while !ordered {
        for rule in rules {
            let (x, y) = rule;
            if row.contains(x) && row.contains(y) {
                let x_index = row.iter().position(|&r| r == *x).unwrap();
                let y_index = row.iter().position(|&r| r == *y).unwrap();
                if y_index < x_index {
                    let temp = row[x_index];
                    row[x_index] = *y;
                    row[y_index] = temp;
                }
            }
        }
        ordered = is_ordered(rules, row);
    }
    row.to_vec()
}