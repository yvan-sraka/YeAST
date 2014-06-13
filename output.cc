#include "output.hh"

//extern std::istream std::cin;

int error(std::string filename, int line_number, std::string message) {
    std::cerr << "\x1b[31m" << "error:" << "\x1b[0m" << " " << filename << ":" << line_number << ": " << message << std::endl;
    return EXIT_FAILURE;
}

int error(std::string filename, std::string message) {
    std::cerr << "\x1b[31m" << "error:" << "\x1b[0m" << " " << filename << ": " << message << std::endl;
    return EXIT_FAILURE;
}

int error(std::string message) {
    std::cerr << "\x1b[31m" << "error:" << "\x1b[0m" << " " << message << std::endl;
    return EXIT_FAILURE;
}

int success(std::string filename, int line_number, std::string message) {
    std::cout << "\x1b[32m" << "success:" << "\x1b[0m" << " " << filename << ":" << line_number << ": " << message << std::endl;
    return EXIT_SUCCESS;
}

int success(std::string filename, std::string message) {
    std::cout << "\x1b[32m" << "success:" << "\x1b[0m" << " " << filename << ": "<< message << std::endl;
    return EXIT_SUCCESS;
}

int success(std::string message) {
    std::cout << "\x1b[32m" << "success:" << "\x1b[0m" << " " << message << std::endl;
    return EXIT_SUCCESS;
}

void warning(std::string filename, int line_number, std::string message) {
    std::cout << "\x1b[33m" << "warning:" << "\x1b[0m" << " " << filename << ":" << line_number << ": " << message << std::endl;
}

void warning(std::string filename, std::string message) {
    std::cout << "\x1b[33m" << "warning:" << "\x1b[0m" << " " << filename << ": " << message << std::endl;
}

void warning(std::string message) {
    std::cout << "\x1b[33m" << "warning:" << "\x1b[0m" << " " << message << std::endl;
}

void note(std::string filename, int line_number, std::string message) {
    std::cout << "\x1b[34m" << "note:" << "\x1b[0m" << " " << filename << ":" << line_number << ": " << message << std::endl;
}

void note(std::string filename, std::string message) {
    std::cout << "\x1b[34m" << "note:" << "\x1b[0m" << " " << filename << ": " << message << std::endl;
}

void note(std::string message) {
    std::cout << "\x1b[34m" << "note:" << "\x1b[0m" << " " << message << std::endl;
}
