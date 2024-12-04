fn main() {
    let lines = include_str!("input.txt");
    let input: Vec<Vec<char>> = lines.lines().map(|line| line.chars().collect()).collect();

    let width = input.len();
    let height = input[0].len();

    part1(&input, width, height);
    part2(&input, width, height);
}

fn part1(input: &[Vec<char>], width: usize, height: usize) {
    let mut answer = 0;
    let word: Vec<char> = "XMAS".chars().collect();
    let word_rev: Vec<char> = word.iter().rev().cloned().collect();

    // Checks horizontally and vertically
    (0..height).for_each(|i| {
        (0..=width - word.len()).for_each(|j| {
            if check_direction(&word, input, i, j, 0) || check_direction(&word_rev, input, i, j, 0)
            {
                answer += 1;
            }
            if check_direction(&word, input, j, i, 1) || check_direction(&word_rev, input, j, i, 1)
            {
                answer += 1;
            }
        });
    });

    // Checks diagonally
    (0..=height - word.len()).for_each(|i| {
        (0..=width - word.len()).for_each(|j| {
            if check_direction(&word, input, i, j, 2) || check_direction(&word_rev, input, i, j, 2)
            {
                answer += 1;
            }
            if check_direction(&word, input, height - i - 1, j, 3)
                || check_direction(&word_rev, input, height - i - 1, j, 3)
            {
                answer += 1;
            }
        });
    });

    println!("Part 1: {}", answer);
}

fn part2(input: &[Vec<char>], width: usize, height: usize) {
    let mut answer = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if input[y][x] == 'A'
                && ((input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S')
                    || (input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M'))
                && ((input[y + 1][x - 1] == 'M' && input[y - 1][x + 1] == 'S')
                    || (input[y + 1][x - 1] == 'S' && input[y - 1][x + 1] == 'M'))
            {
                answer += 1;
            }
        }
    }

    println!("Part 2: {}", answer);
}

/*
Checks if the word can be found in the input in the specified direction.
    0: Horizontal
    1: Vertical
    2: Diagonal (top left to bottom right)
    3: Diagonal (bottom left to top right)
 */
fn check_direction(
    word: &[char],
    input: &[Vec<char>],
    y: usize,
    x: usize,
    direction: usize,
) -> bool {
    if direction == 0 {
        (0..word.len()).all(|i| input[y][x + i] == word[i])
    } else if direction == 1 {
        return (0..word.len()).all(|i| input[y + i][x] == word[i]);
    } else if direction == 2 {
        return (0..word.len()).all(|i| input[y + i][x + i] == word[i]);
    } else {
        return (0..word.len()).all(|i| input[y - i][x + i] == word[i]);
    }
}
