package main

import rl "vendor:raylib"

Block :: rl.Vector2

// Direction :: enum{Up, Down, Left, Right}

Snake :: struct {
	body:                 [dynamic]Block,
	// direction:            Direction,
	is_dead:              bool,
	is_length_increasing: bool,
}

Apple :: Block

AppState :: struct {
	snake: Snake,
	apple: Apple,
	score: u32,
}

BOARD_WIDTH :: 17
BOARD_HEIGHT :: 15
GRID_COLOR_DARK :: rl.Color{95, 167, 47, 255}
GRID_COLOR_LIGHT :: rl.Color{119, 201, 64, 255}
CELL_SIZE :: 50

snake_init :: proc() -> Snake {
	return Snake {
		body                 = make([dynamic]Block, 0, 1),
		// direction            = .Right,
		is_dead              = false,
		is_length_increasing = false,
	}
}

apple_init :: proc() -> Apple {
	return Apple{0, 0}
}

startup :: proc() -> AppState {
	return AppState{snake = snake_init(), apple = apple_init(), score = 0}
}

draw_background :: proc() {
	for y := 0; y < BOARD_HEIGHT; y += 1 {
		for x := 0; x < BOARD_WIDTH; x += 1 {
			position := rl.Vector2{f32(x) * CELL_SIZE, f32(y) * CELL_SIZE}
			size := rl.Vector2{CELL_SIZE, CELL_SIZE}
			color := x % 2 == y % 2 ? GRID_COLOR_DARK : GRID_COLOR_LIGHT

			rl.DrawRectangleV(position, size, color)
		}
	}
}

game_draw :: proc(state: AppState) {
	rl.BeginDrawing()
	defer rl.EndDrawing()
	rl.ClearBackground(rl.RAYWHITE)
	draw_background()
}

windowWidth :: 800
windowHeight :: 600
windowTitle :: "Snake"

main :: proc() {
	rl.InitWindow(windowWidth, windowHeight, windowTitle)
	defer rl.CloseWindow()
	rl.SetTargetFPS(120)

	state := startup()

	for !rl.WindowShouldClose() {
		game_draw(state)
	}
}
