NAME := "nya"
SRCS := lib.rs
DIRC := src/
LSTC := $(patsubst %,$(DIRC)%,$(SRCS))

.SILENT: all
.PHONY: default all

default: all

all:
	git submodule init
	git submodule update
	gcc -shared -o libnya.dylib src/lib.c -Iinclude
