NAME := "nya"
SRCS := lib.rs
DIRC := src/
LSTC := $(patsubst %,$(DIRC)%,$(SRCS))
UNAME := $(shell uname)

.SILENT: all
.PHONY: default all

default: all


all:
	git submodule init
	git submodule update
ifeq ($(UNAME),Linux)
	gcc -fPIC -shared -o libnya.dylib src/lib.c -Iinclude
else
	gcc -shared -o libnya.dylib src/lib.c -Iinclude
endif
