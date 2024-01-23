fn main() {
    let test_cases = include_str!("./input1.txt");
    let output = part1(test_cases.trim());
    dbg!(output);
}

fn part1(test_cases: &str) -> u32 {
    //let configs: Vec<&str> = test_cases.split("\n").collect();
    //let mut sum: u32 = 0;

    //for config in configs.iter() {
    //    let mut first = 'a';
    //    for letter in config.chars() {
    //        if letter.is_numeric() {
    //            first = letter;
    //            break;
    //        }
    //    }

    //    let mut last = 'a';
    //    for letter in config.chars().rev() {
    //        if letter.is_numeric() {
    //            last = letter;
    //            break;
    //        }
    //    }

    //    sum += (first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()) as u32;
    //}

    //sum

    // PRO METHOD
    test_cases
        // Takes a multiline &str and returns an iterator where each item is a line from the
        // original string.
        .lines()
        // Map every element to something.
        .map(|line| {
            // filter_map() takes a closure that returns an Option<T>. If Some(T) is returned, that
            // object is accepted by the filter, if None is returned, it is filtered.
            // So here, every non-integer character is filtered and the remaining integer
            // characters are mapped to their u32 self.
            let mut it = line.chars().filter_map(|letter| letter.to_digit(10));
            // Since we know that every input will have atleast 1 number, we can go ahead with this
            // approach of using expect().
            let first = it.next().expect("[ERROR] why is this not a number?");

            // We match it.last() because if the string has only 1 digit, then it.last() will
            // return None as the iterator it will have only 1 element.
            match it.last() {
                Some(last_num) => first * 10 + last_num,
                None => first * 10 + first,
            }
        })
        // To sum up all the values in the new test_cases of type Iterator<Item = u32>.
        .sum()
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
