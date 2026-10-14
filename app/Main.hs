{-# LANGUAGE TemplateHaskell #-}

module Main (main) where

import Linear.V2 (V2)

import Raylib.Core (clearBackground, closeWindow, initWindow, setTargetFPS, windowShouldClose)
import Raylib.Core.Text (drawText)
import Raylib.Types.Core (Vector2)
import Raylib.Util (drawing, raylibApplication, WindowResources)
import Raylib.Util.Colors (lightGray, rayWhite)

type Block = Vector2

-- data Direction = Up | Down | Left | Right

data Snake = Snake
    { snakeBody :: [Block],
      -- snakeDirection :: Direction,
      snakeIsDead :: Bool,
      snakeIsLengthIncreasing :: Bool
    }

type Apple = Block

data AppState = AppState
    { stateWindow :: WindowResources,
      stateSnake :: Snake,
      stateApple :: Apple,
      stateScore :: Word
    }

snakeInit :: Snake
snakeInit = Snake
    { snakeBody = [],
      -- snakeDirection = Right,
      snakeIsDead = False,
      snakeIsLengthIncreasing = False
    }

appleInit :: Apple
appleInit = V2 0 0

startup :: IO AppState
startup = do
    window <- initWindow windowWidth windowHeight windowTitle 
    setTargetFPS 120
    return (AppState {
        stateWindow = window,
        stateSnake = snakeInit,
        stateApple = appleInit,
        stateScore = 0
    })
  where
    windowWidth = 800
    windowHeight = 600
    windowTitle = "Snake"

mainLoop :: AppState -> IO AppState
mainLoop state = drawing drawGraphics >> return state
    where
        drawGraphics = do
            clearBackground rayWhite
            drawText "Hello, Raylib!" 30 40 18 lightGray

shouldClose :: AppState -> IO Bool
shouldClose _ = windowShouldClose

teardown :: AppState -> IO ()
teardown = closeWindow . Just . stateWindow

$(raylibApplication 'startup 'mainLoop 'shouldClose 'teardown)
