#include <iostream>
#include <fstream>
#include <string>
#include <vector>

#define FILE "input.txt"

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

    std::cout << _lines_ << "\n";

    for (const auto& line: input) {
        std::cout << line << "\n";
    }

    int sum = 0;
    const std::string NUMS = "123456789";
    for (const auto& config: input) {
        std::cout << "For " << config << ":\n";
        std::size_t first_num = config.find_first_of(NUMS);
        std::size_t last_num = config.find_last_of(NUMS);

        int num = ((config[first_num] - '0') * 10) + (config[last_num] - '0');
        std::cout << "\tNum formed: " << num << "\n";
        sum += num;
    }

    std::cout << "Sum: " << sum << std::endl;

}