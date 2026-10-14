package main

import rl "vendor:raylib"

Block :: rl.Vector2

// Direction :: enum{Up, Down, Left, Right}

Snake :: struct {
	body:                 [dynamic]Block,
	// direction: Direction,
	is_dead:              bool,
	is_length_increasing: bool,
}

Apple :: Block

AppState :: struct {
	snake: Snake,
	apple: Apple,
	score: u32,
}

snake_init :: proc() -> Snake {
	return Snake {
		body                 = make([dynamic]Block, 0, 1),
		// direction = .Right,
		is_dead              = false,
		is_length_increasing = false,
	}
}

apple_init :: proc() -> Apple {
	return Apple{0, 0}
}

startup :: proc() -> AppState {
	windowWidth :: 800
	windowHeight :: 600
	windowTitle :: "Snake"

	rl.InitWindow(windowWidth, windowHeight, windowTitle)
	rl.SetTargetFPS(120)

	return AppState{snake = snake_init(), apple = apple_init(), score = 0}
}

main_loop :: proc(state: AppState) {
	rl.BeginDrawing()
	defer rl.EndDrawing()
	rl.ClearBackground(rl.RAYWHITE)
	rl.DrawText("Hello, Raylib!", 30, 40, 18, rl.LIGHTGRAY)
}

should_close :: proc(state: AppState) -> bool {
	return rl.WindowShouldClose()
}

teardown :: proc(state: AppState) {
	rl.CloseWindow()
}

main :: proc() {
	state := startup()
	defer teardown(state)

	for !should_close(state) {
		main_loop(state)
	}
}
