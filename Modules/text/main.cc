#include <iostream>

int main() {
    std::string str;
    std::cin >> str;
    for (size_t pos = 0; (pos = str.find("\\", pos)) != std::string::npos; pos += 2)
        str.replace(pos, 1, "\\\\");
    for (size_t pos = 0; (pos = str.find("\"", pos)) != std::string::npos; pos += 2)
        str.replace(pos, 1, "\\\"");
    for (size_t pos = 0; (pos = str.find("\n", pos)) != std::string::npos; pos += 5)
        str.replace(pos, 1, "\" \\\n\"");
    this->code_ += "\"" + str + "\\n\" \\\n";
    std:cout << std::endl;
}
