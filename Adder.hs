{-# LANGUAGE ForeignFunctionInterface #-}
module Adder where

import Foreign.C
import Foreign.C.Types

data Attribute = NoneAttribute | Bold | Dim | Italic | Underline | Blink | Reverse | Hidden deriving (Enum)

data Sheet = NoneSheet | Bust deriving (Enum)

data Cardinal = UpperLeft | UpperMiddle | UpperRight | MiddleLeft | MiddleCentral | MiddleRight | LowerLeft | LowerMiddle | LowerRight deriving (Enum)

data Part = NonePart | ArmLeft | ArmRight | Boobs | Clavicle | EarLeft | EarRight | EyeLeft | EyeRight | HairTop | HairLeft | HairRight | HandLeft | HandRight | Mouth | Tail | Bell | ExclamationMark | ExclamationMarks | Heart | Hearts | Lantern | QuestionMark | QuestionMarks | WoolBall

data Emotion = NoneEmotion | Angry | Happy | Love | Malicious | Misunderstanding | Shocked | Sleepy | Speechless

instance Enum Emotion where
  fromEnum NoneEmotion = 95
  fromEnum Angry = 97
  fromEnum Happy = 104
  fromEnum Love = 108
  fromEnum Malicious = 109
  fromEnum Misunderstanding = 105
  fromEnum Shocked = 111
  fromEnum Sleepy = 115
  fromEnum Speechless = 101
 
  toEnum 95 = NoneEmotion
  toEnum 97 = Angry
  toEnum 104 = Happy
  toEnum 108 = Love
  toEnum 109 = Malicious
  toEnum 105 = Misunderstanding
  toEnum 111 = Shocked
  toEnum 115 = Sleepy
  toEnum 101 = Speechless

data Position = Position { cardinal :: Cardinal
                         , cartesian :: (CUShort, CUShort)
}

data Tuple = Tuple { part :: Part
                   , emotion :: Emotion
}

adder :: CInt -> CInt -> IO CInt
adder x y = return $ x + y

foreign export ccall adder :: CInt -> CInt -> IO CInt
