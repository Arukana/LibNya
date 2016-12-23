[![Build Status](https://travis-ci.org/Arukana/libnya.svg?branch=haskell)](https://travis-ci.org/Arukana/libnya)

gcc -shared -o libss.so -fPIC libss.c libadder.dylib -I"$(ghc --print-libdir)/include"
install_name_tool -change "@rpath/libadder.dylib" "@executable_path/libadder.dylib" libss.so
gcc -o main main.c
