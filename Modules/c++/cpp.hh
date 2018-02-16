#ifndef TEXT_HH
# define TEXT_HH

#include <string>

#include "Bloc.hh"

class Cpp : public Bloc
{
public:
    Cpp(std::string environment);
    void feed(const std::string &code);
};

#endif // !TEXT_HH
