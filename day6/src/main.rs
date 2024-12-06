use std::collections::HashSet;

fn main() {
    let (grid, height, width, x, y) = parse();

    part1(grid.clone(), height, width, x, y);
    part2(grid.clone(), height, width, x, y);
}

fn part1(grid: Vec<Vec<char>>, height: usize, width: usize, mut x: usize, mut y: usize) {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_index = 0;
    let mut visited = HashSet::new();

    while x < height && y < width {
        visited.insert((x, y));
        loop {
            let direction = directions[dir_index];
            let next_x = (x as isize + direction.0) as usize;
            let next_y = (y as isize + direction.1) as usize;
            if next_x < height && next_y < width && grid[next_x][next_y] == '#' {
                dir_index = (dir_index + 1) % 4;
            } else {
                x = next_x;
                y = next_y;
                break;
            }
        }
    }
    println!("Part 1: {}", visited.len());
}

fn part2(mut grid: Vec<Vec<char>>, height: usize, width: usize, x: usize, y: usize) {
    let mut answer = 0;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '#' || grid[i][j] == '^' || grid[i][j] == 'X' {
                continue;
            }

            grid[i][j] = 'X';

            let mut visited = HashSet::new();
            let (mut current_direction, mut current_x, mut current_y) = (0, x, y);

            while current_x < height && current_y < width && !visited.contains(&(current_x, current_y, current_direction)) {
                visited.insert((current_x, current_y, current_direction));
                loop {
                    let current_dir = directions[current_direction];
                    let next_x = (current_x as isize + current_dir.0) as usize;
                    let next_y = (current_y as isize + current_dir.1) as usize;
                    if next_x < height && next_y < width && (grid[next_x][next_y] == '#' || grid[next_x][next_y] == 'X') {
                        current_direction = (current_direction + 1) % 4;
                    } else {
                        current_x = next_x;
                        current_y = next_y;
                        break;
                    }
                }
            }

            if visited.contains(&(current_x, current_y, current_direction)) {
                answer += 1;
            }

            grid[i][j] = '.';
        }
    }

    println!("Part 2: {}", answer);
}

fn parse() -> (Vec<Vec<char>>, usize, usize, usize, usize) {
    let grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|r| r.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut x = 0;
    let mut y = 0;
    (0..height).for_each(|i| {
        (0..width).for_each(|j| {
            if grid[i][j] == '^' {
                x = i;
                y = j;
            }
        });
    });

    (grid, height, width, x, y)
}
