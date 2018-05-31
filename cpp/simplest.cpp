#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath>

bool is_number(const std::string& s)
{
    return !s.empty() && std::find_if(s.begin(), 
        s.end(), [](char c) { return !std::isdigit(c); }) == s.end();
}

int main()
{
    std::string input;
    std::cout << "Insert a number to know if we like it: ";
    std::cin >> input;
    if (!is_number(input)) {
        std::cerr << "Input is not a number!" << std::endl;
        return 1;
    }
    if (input.size() < 2) {
        std::cerr << "Input must have lenght greater than 1" << std::endl;
        return 1;
    }
    for (int i = 0; i < input.size()-1; i++) {
        if (std::abs(input[i] - input[i+1]) != 1) {
            std::cout << "We don't like this number :c" << std::endl;
            return 0;
        }
    }
    std::cout << "We like this number!" << std::endl;
    return 0;
}