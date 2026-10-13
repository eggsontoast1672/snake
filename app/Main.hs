module Main (main) where

type Block = Vector2

data Direction = Up | Down | Left | Right

newtype Snake = Snake
    { snakeBody :: [Block],
      snakeDirection :: Direction,
      snakeIsDead :: Bool,
      snakeIsLengthIncreasing :: Bool
    }

type Apple = Block

-- State:
--
-- snake
-- apple
-- score

main :: IO ()
main = putStrLn "Hello, Haskell!"
