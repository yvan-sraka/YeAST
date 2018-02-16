#ifndef TEXT_HH
# define TEXT_HH

#include <string>

#include "Bloc.hh"

class Text : public Bloc
{
public:
    Text(std::string environment);
    void feed(const std::string &code) override;
};

#endif // !TEXT_HH
