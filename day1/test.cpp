#include <iostream>
#include <vector>
#include <ranges>

/*
 * Here's me learning about ranges and views. Modern c++. yay.
*/

int main() {
    std::vector<int> range = {1,2,3,4,5,6,7,8};
    auto less_than_three = range |  std::views::drop_while([](int x) { return x <= 3; });
    for (const auto& num: less_than_three) {
        std::cout << num << "\n";
    }
}