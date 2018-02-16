#include "cpp.hh"

Cpp::Cpp(std::string environment)
:Bloc(environment) {
}

void Cpp::feed(const std::string &code) {
    this->code_ += code + "\n";
}
