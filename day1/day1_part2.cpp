#include <iostream>
#include <fstream>
#include <string>
#include <ranges>
#include <vector>
#include <array>

#define FILE "input.txt"
#define NAN -1

namespace {
    std::array<std::string, 19> numbers = {
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    };

    int to_digit(std::string_view str) {
        auto digit_match = std::ranges::find_if(numbers, [&](auto& digit) {
            return str.starts_with(digit);
        });

        if (digit_match == numbers.end()) {
            return NAN;
        }

        // digit will give contain the ending index value of the digit word.
        auto digit = std::distance(numbers.begin(), digit_match);
        if (digit >= 10) {
            return digit - 9;
        }
        return digit;
    }
}

int main() {
    std::ifstream input_file(FILE);
    if (!input_file.is_open()) {
        std::cerr << "File not opened!" << std::endl;
        return 1;
    }

    int _lines_ = 0;
    std::string temp;
    std::vector<std::string> input;
    while (std::getline(input_file, temp)) {
        _lines_++;
        input.push_back(temp);
    }

    int sum = 0;

    for (const auto& line: input) {
        int first_digit = NAN;
        int last_digit = NAN;

        for (auto position: std::views::iota(line.begin(), line.end())) {
            auto sub_line = std::string_view(position, line.end());

            if (int digit = to_digit(sub_line); digit != NAN) {
                if (first_digit == NAN) {
                    first_digit = digit;
                }
                last_digit = digit;
            }
        }

        sum += ((first_digit * 10) + last_digit);
    }

    std::cout << sum << "\n";
}