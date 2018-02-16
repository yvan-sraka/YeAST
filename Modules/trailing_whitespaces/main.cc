#include <iostream>

int main() {
    std::string str;
    std::cin >> str;
    for (size_t pos = 0; (pos = str.find("\n", pos)) != std::string::npos; pos += 5)

    std:cout << str;
}
