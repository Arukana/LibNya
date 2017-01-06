PYTHON_LIB := /usr/lib

.SILENT: all
.PHONY: default all

default: all

all:
	git submodule init
	git submodule update
	gcc -shared -o libnya.dylib -fPIC -Iinclude $(python-config --cflags) $(python-config --ldflags) src/lib.c
#	gcc -shared -o libnya.dylib -fPIC -Iinclude $(python-config --cflags) $(python-config --ldflags) src/lib.c
