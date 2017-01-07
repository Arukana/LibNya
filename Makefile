CFLAGS := $(shell python-config --cflags)
LDFLAG := $(shell python-config --ldflags)

.SILENT: all
.PHONY: default all

default: all

all:
	git submodule init
	git submodule update
	gcc -shared -o libnya.dylib -fPIC -Iinclude $(CFLAGS) $(LDFLAG) src/lib.c
