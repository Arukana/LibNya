NAME := "nya"
SRCS := lib.rs
DIRC := src/
LSTC := $(patsubst %,$(DIRC)%,$(SRCS))

.SILENT: all
.PHONY: default all

default: all

all:
#	git submodule init
#	git submodule update
	rustc --verbose src/lib.rs --crate-name $(NAME) --crate-type dylib --out-dir .
