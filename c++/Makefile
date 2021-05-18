CC=g++
CXX=g++
CXXFLAGS=-g -Wall
LDFLAGS=
LDLIBS=

targets=lrvalues

.PHONY: default
default: $(targets)

$(foreach t, $(targets), $(t)):
$(foreach t, $(targets), $(t).o):

.PHONY: help
help:
	@echo "Possible targets: $(targets)"


.PHONY: clean
clean:
	rm -rf *.o $(targets)
