fn main() {
    let test_cases = include_str!("./input1.txt");
    let output = part2(test_cases.trim());
    dbg!(output);
}

fn part2(test_cases: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first = 0;
    let mut last = 0;
    let mut ans: u32 = 0;
    for line in test_cases.lines() {
        'one: for (idx, letter) in line.chars().enumerate() {
            if letter.is_numeric() {
                first = letter.to_digit(10).unwrap();
                break;
            } else {
                for i in 3..=5 {
                    if idx + i >= line.len() {
                        continue;
                    }
                    let subs = &line[idx..(idx + i)];
                    if numbers.contains(&subs) {
                        first = numbers.iter().position(|num| num == &subs).unwrap() as u32 + 1u32;
                        break 'one;
                    }
                }
            }
        }

        let line_rev = line.chars().rev().collect::<String>();
        'two: for (idx, letter) in line_rev.chars().enumerate() {
            if letter.is_numeric() {
                last = letter.to_digit(10).unwrap();
                break;
            } else {
                for i in 3..=5 {
                    if idx + i >= line.len() {
                        continue;
                    }
                    let subs = &line_rev[idx..(idx + i)];
                    let subs = subs.chars().rev().collect::<String>();
                    if numbers.contains(&subs.as_str()) {
                        last = numbers
                            .iter()
                            .position(|num| *num == subs.as_str())
                            .unwrap() as u32
                            + 1u32;
                        break 'two;
                    }
                }
            }
        }
        ans += first * 10 + last;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
dmqvplslvxgbd2",
        );
        assert_eq!(result, 303u32);
    }
}

// registered post
// weigh the letter, get corresponding stamp times 2.
//
