fn main() {
    let test_cases = include_str!("./input1.txt");
    let output = part1(test_cases.trim());
    dbg!(output);
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_symbol(character: char) -> bool {
    !character.is_numeric() && character != '.'
}

// What I've done is basically:
// - Made a character grid of Vec<Vec<char>> out of the input &str.
// - Made a symbols grid such that, if there is a symbol at (x, y) in the grid i made above, then symbol_grid[x][y] = true, else false.
// - Now I iterate over the character grid, and for each character I check if it's a '.' or a number.
// - If it is a '.', I continue.
// - If it is a number, I find the entire number. Like for eg, in the sequence ...'3','4','5'..., I will first come across 3. I find the rest of the numbers with a while loop and an idx, and form 345 there itself.
// - Now I know where the number started {col - 1} and where the number ends {idx}.
// - I can check all the neighbouring positions to check for symbols from the symbols grid.
// - If symbol is adjacent, I add the number.

fn part1(input: &str) -> u32 {
    let grid = make_grid(input);
    let m = grid.len();
    let n = grid[0].len();
    let mut symbol_grid = vec![vec![false; n]; m];

    for (row, line) in grid.iter().enumerate() {
        for (col, &character) in line.iter().enumerate() {
            if is_symbol(character) {
                symbol_grid[row][col] = true;
            }
        }
    }

    let mut sum: u32 = 0;

    for (row, line) in grid.iter().enumerate() {
        println!("In row {row}, line: {:?}", line);
        let mut skip_iterations = (false, usize::MAX);
        for (col, &character) in line.iter().enumerate() {
            if character == '.' || col == skip_iterations.1 {
                skip_iterations.0 = false;
                continue;
            }
            if skip_iterations.0 {
                continue;
            }

            if character.is_numeric() {
                println!("numeric character found: {character}");
                skip_iterations.0 = true;
                let mut idx = col;
                let mut number = 0;
                while idx < n && line[idx].is_numeric() {
                    number = number * 10u32 + line[idx].to_digit(10).unwrap();
                    idx += 1;
                }
                skip_iterations.1 = idx;
                // let temp = line.get(idx).unwrap();
                println!("dimensions are {m}x{n}");
                println!("number formed: {number}");
                // println!("idx is {idx}, character at idx: {temp}");
                //
                if idx == n {
                    idx = idx - 1;
                }

                'outer: for i in (row as i32 - 1i32)..=(row as i32 + 1i32) {
                    for j in (col as i32 - 1i32)..=idx as i32 {
                        if i >= 0
                            && j >= 0
                            && i < m as i32
                            && i < n as i32
                            && symbol_grid[i as usize][j as usize]
                        {
                            println!("=========={number} IS ADDED===========");
                            sum += number;
                            break 'outer;
                        }
                    }
                }

                println!("Sum after this line: {sum}\n");
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361u32);
    }
}
