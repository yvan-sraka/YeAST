#ifndef OUTPUT_HH
# define OUTPUT_HH

#include <iostream>
#include <string>

int error(std::string filename, int line_number, std::string message);
int error(std::string filename, std::string message);
int error(std::string message);

int success(std::string filename, int line_number, std::string message);
int success(std::string filename, std::string message);
int success(std::string message);

void warning(std::string filename, int line_number, std::string message);
void warning(std::string filename, std::string message);
void warning(std::string message);

void note(std::string filename, int line_number, std::string message);
void note(std::string filename, std::string message);
void note(std::string message);

#endif // !OUTPUT_HH
