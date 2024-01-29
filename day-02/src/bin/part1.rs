fn main() {
    let test_cases = include_str!("./input1.txt");
    let output = part1(test_cases.trim());
    dbg!(output);
}

fn part1(test_cases: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142u32)
    }
}
