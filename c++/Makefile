CC=g++
CXX=g++
CXXFLAGS=-g -Wall
LDFLAGS=
LDLIBS=

tests=lrvalues

.PHONY: default
default: $(tests)

$(foreach t, $(tests), $(t)):
$(foreach t, $(tests), $(t).o):

.PHONY: clean
clean:
	rm -rf *.o $(tests)
