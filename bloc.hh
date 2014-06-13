#ifndef BLOC_HH
# define BLOC_HH

#include <stdio.h>

#include <iostream>
#include <fstream>
#include <string>

class Bloc
{
public:
    Bloc(std::string environment);
    virtual ~Bloc();

    virtual void feed(const std::string &code);
    virtual std::string extract(void);

private:
    std::string code_;
    std::string environment_;
    std::string filename_;
};

#endif // !BLOC_HH
