// use std::collections::HashMap;

use core::panicking::panic;

fn main() {
    let test_cases = include_str!("./input1.txt");
    let output = part2(test_cases.trim());
    dbg!(output);
}

fn part2(test_cases: &str) -> u32 {
    // let word_to_num: HashMap<&str, u32> = HashMap::new();
    // word_to_num.insert("one", 1);
    // word_to_num.insert("two", 2);
    // word_to_num.insert("three", 3);
    // word_to_num.insert("four", 4);
    // word_to_num.insert("five", 5);
    // word_to_num.insert("six", 6);
    // word_to_num.insert("seven", 7);
    // word_to_num.insert("eight", 8);
    // word_to_num.insert("nine", 9);

    // to access the number, do vec[number.index - 9]. eg vec[9 - 9] = "1"; vec[9] = "one".
    let word_to_num: Vec<&str> = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    // {THIS CODE IS WRONG BECAUSE WHAT IF 1 COMES BEFORE ANY WORD?}

    // for line in test_cases.lines() {
    //     let mut first = None;
    //     'out: for i in 0..line.len() {
    //         for j in 9..18 {
    //             if i + word_to_num[j].len() > line.len() {
    //                 continue;
    //             }

    //             if line[i..i + word_to_num[j].len()] == **word_to_num[j] {
    //                 first = Some(word_to_num[j - 9]);
    //                 break 'out;
    //             }
    //         }
    //     }
    //     let Some(first) = first else { panic("invalid inp"); };

    //     let mut last = None;
    //     'out: for i in (0..line.len()).rev() {
    //         for j in 9..18 {
    //             if i + word_to_num[j].len() > line.len() {
    //                 continue;
    //             }

    //             if line[i..i + word_to_num[j].len()] == **word_to_num[j] {
    //                 last = Some(word_to_num[j - 9]);
    //                 break 'out;
    //             }
    //         }
    //     }
    //     let Some(last) = last else { panic("invalid inp"); };
    // }
}

// todo: write the test again my god.
