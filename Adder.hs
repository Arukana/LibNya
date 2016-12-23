{-# LANGUAGE ForeignFunctionInterface #-}
module Adder where

import Foreign.C

adder :: CInt -> CInt -> IO CInt
adder x y = return $ x + y

foreign export ccall adder :: CInt -> CInt -> IO CInt
