#include <iostream>
#include <fstream>
#include <string>
#include <stack>

#include "output.hh"
#include "bloc.hh"

int main(int argc, char *argv[]) {
    if (argc <= 1) // Bad
        return error("invalid number of arguments"); // -> Show Help
    std::string filename = argv[1];
    std::ifstream file(filename);
    if (!file.is_open())
        return error(filename, "unable to open file");
    std::string line;
    std::stack<Bloc> stack;
    for (int line_number = 1; getline(file, line); ++line_number) {
        std::size_t shebang = line.find("#!");
        std::size_t bangshe = line.find("!#");
        if (shebang != std::string::npos) {
            if (bangshe == std::string::npos) {
                if (!stack.empty())
                    stack.top().feed(line.substr(0, shebang)); //
                stack.push(Bloc(line.substr(shebang + 2))); //
            } else { // bangshe != std::string::npos
                return error(filename, line_number, "'#!' and '!#' on same line"); // Bad
            }
        } else { // shebang == std::string::npos
            if (bangshe != std::string::npos) {
                if (stack.empty())
                    return error(filename, line_number, "unexpected '!#'");
                Bloc state = stack.top();
                stack.pop();
                if (stack.empty()) {
                    state.extract();
                    return EXIT_SUCCESS; //return success("build finish without errors!");
                } else {
                    stack.top().feed(state.extract());
                    stack.top().feed(line.substr(bangshe + 2));
                }
            } else if (!stack.empty()) {
                stack.top().feed(line);
            } else { // if (line_number == 1)
                warning(filename, line_number, "first line should start by '#!' to set environment"); // #! was not set in top of this file
            } // else -> Ignored content...
        }
    }
    file.close();
    return EXIT_SUCCESS;
}
