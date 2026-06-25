use std::process::Termination;

use raylib::prelude::*;

const TILE_SIZE: i32 = 32;
const BOARD_WIDTH: i32 = 10 * TILE_SIZE;
const BOARD_HEIGHT: i32 = 20 * TILE_SIZE;
const BOARD_OFFSET_X: i32 = (SCREEN_WIDTH - BOARD_WIDTH) / 2;
const BOARD_OFFSET_Y: i32 = (SCREEN_HEIGHT - BOARD_HEIGHT) / 2;
const BOARD_COLUMNS: usize = 10;
const BOARD_ROWS: usize = 20;
const BOARD_SIZE: usize = 10 * 20;
const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 800;

fn draw_block(d: &mut RaylibDrawHandle, x: i32, y: i32, color: Color) {
    let px = BOARD_OFFSET_X + x * TILE_SIZE;
    let py = BOARD_OFFSET_Y + y * TILE_SIZE;

    d.draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, color);
    d.draw_rectangle_lines(px, py, TILE_SIZE, TILE_SIZE, Color::BLUE);
}

fn draw_board(d: &mut RaylibDrawHandle<'_>, board: &Board) {
    for y in 0..BOARD_ROWS {
        for x in 0..BOARD_COLUMNS {
            match board.cell(x, y) {
                Some(cell) => draw_block(d, x as i32, y as i32, cell.kind.color()),
                None => draw_block(d, x as i32, y as i32, Color::new(35, 35, 35, 255)),
            }
        }
    }
}

#[derive(Clone, Copy)]
enum TetrominoKind {
    I, O, T, S, Z, J, L,
}

impl TetrominoKind {
    fn color(self) -> Color {
        match self {
            TetrominoKind::I => Color::new(0, 240, 240, 255),
            TetrominoKind::O => Color::new(240, 240, 0, 255),
            TetrominoKind::T => Color::new(160, 0, 240, 255),
            TetrominoKind::S => Color::new(0, 240, 0, 255),
            TetrominoKind::Z => Color::new(240, 0, 0, 255),
            TetrominoKind::J => Color::new(0, 0, 240, 255),
            TetrominoKind::L => Color::new(240, 160, 0, 255),
        }
    }
}

#[derive(Clone, Copy)]
struct Cell {
    kind: TetrominoKind,
}

type Grid = [Option<Cell>; BOARD_SIZE];

struct Board {
    cells: Grid,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [None; BOARD_SIZE],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * BOARD_COLUMNS + x
    }

    fn cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= BOARD_COLUMNS || y >= BOARD_ROWS {
            return None;
        }

        let index = self.index(x, y);
        self.cells[index].as_ref()
    }

    fn set_cell(&mut self, x: usize, y: usize, cell: Option<Cell>) {
        if x >= BOARD_COLUMNS || y >= BOARD_ROWS {
            return;
        }
        let index = self.index(x, y);
        self.cells[index] = cell;
    }
}


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Tetris")
        .build();

    let mut board = Board::new();
    board.set_cell(4, 18, Some(Cell { kind: TetrominoKind::T}));
    board.set_cell(5, 18, Some(Cell { kind: TetrominoKind::T}));
    board.set_cell(6, 18, Some(Cell { kind: TetrominoKind::T}));
    board.set_cell(5, 19, Some(Cell { kind: TetrominoKind::T}));
    
    while !rl.window_should_close() {

        rl.draw(&thread, |mut d| {
            d.clear_background(Color::BLACK);
            d.draw_text("Tetris", SCREEN_WIDTH / 2 - 30, 10, 20, Color::YELLOW);

            draw_board(&mut d, &board);
        });
    }
}