use std::collections::HashSet;

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut antennas:Vec<(usize, usize, char)> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((x, y, c));
            }
        }
    }

    // let lines: Vec<&str> = lines.collect();
    println!("Part 1: {}", part1(&antennas, &lines));
    println!("Part 2: {}", part2(&antennas, &lines));
}

fn part1(antennas: &[(usize, usize, char)], lines: &[&str]) -> usize {
    let mut antinodes = HashSet::new();

    // Find pairs of antennas with the same frequency
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (x1, y1, f1) = antennas[i];
            let (x2, y2, f2) = antennas[j];

            if f1 == f2 {
                // Calculate the positions of the antinodes
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Antinode positions
                let ax1 = x1 as isize - dx;
                let ay1 = y1 as isize - dy;
                let ax2 = x2 as isize + dx;
                let ay2 = y2 as isize + dy;

                // Add valid antinode positions to the set
                if ax1 >= 0 && ay1 >= 0 && ax1 < lines[0].len() as isize && ay1 < lines.len() as isize {
                    antinodes.insert((ax1 as usize, ay1 as usize));
                }
                if ax2 >= 0 && ay2 >= 0 && ax2 < lines[0].len() as isize && ay2 < lines.len() as isize {
                    antinodes.insert((ax2 as usize, ay2 as usize));
                }
            }
        }
    }

    antinodes.len()
}

fn part2(antennas: &[(usize, usize, char)], lines: &[&str]) -> usize {
    let mut antinodes = HashSet::new();

    // Group antennas by frequency
    let mut frequency_map = std::collections::HashMap::new();
    for &(x, y, f) in antennas {
        frequency_map.entry(f).or_insert(Vec::new()).push((x, y));
    }

    // Calculate antinodes for each frequency group
    for (_, positions) in frequency_map {
        let n = positions.len();
        for i in 0..n {
            for j in i + 1..n {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                // Calculate the positions of the antinodes
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Antinode positions
                let mut k = 1;
                while x1 as isize + k * dx >= 0
                    && y1 as isize + k * dy >= 0
                    && (x1 as isize + k * dx) < lines[0].len() as isize
                    && (y1 as isize + k * dy) < lines.len() as isize
                {
                    antinodes.insert(((x1 as isize + k * dx) as usize, (y1 as isize + k * dy) as usize));
                    k += 1;
                }

                k = 1;
                while x1 as isize - k * dx >= 0
                    && y1 as isize - k * dy >= 0
                    && (x1 as isize - k * dx) < lines[0].len() as isize
                    && (y1 as isize - k * dy) < lines.len() as isize
                {
                    antinodes.insert(((x1 as isize - k * dx) as usize, (y1 as isize - k * dy) as usize));
                    k += 1;
                }
            }
        }

        // Add the positions of the antennas themselves as antinodes
        for &(x, y) in &positions {
            antinodes.insert((x, y));
        }
    }

    antinodes.len()
}
