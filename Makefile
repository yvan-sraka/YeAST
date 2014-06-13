CXX=clang++
CXXFLAGS=-Wall -Werror -pedantic -std=c++11

SRC=bloc.cc output.cc main.cc

all: $(SRC:.cc=.o)
	$(CXX) $(CXXFLAGS) -o yeast $^

clean:
	rm -f *.o
	rm -f yeast
