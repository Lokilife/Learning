use std::{io::Stdout, ops::{Add, Sub}};
use crossterm::{cursor, execute, terminal};

#[derive(PartialEq, Clone, Copy)]
pub struct Vector2i {
    x: i32,
    y: i32,
}
impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_usize(val: &usize, width: &usize) -> Self {
        Self { x: (val % width) as i32, y: (val / width) as i32 }
    }

    pub fn to_usize(&self, width: &usize) -> usize {
        return self.x as usize + self.y as usize * width;
    }

    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const LEFT:  Self = Self { x: -1, y: 0 };
    pub const UP:    Self = Self { x: 0, y: -1 };
    pub const DOWN:  Self = Self { x: 0, y: 1 };
}
impl Sub<Vector2i> for Vector2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Vector2i::new(self.x - rhs.x, self.y - rhs.y);
    }
}
impl Add<Vector2i> for Vector2i {
    type Output = Self;

    fn add(self, rhs: Vector2i) -> Self::Output {
        return Vector2i::new(self.x + rhs.x, self.y + rhs.y);
    }
}
impl Add<&Vector2i> for Vector2i {
    type Output = Self;

    fn add(self, rhs: &Vector2i) -> Self::Output {
        return Vector2i::new(self.x + rhs.x, self.y + rhs.y);
    }
}

pub fn clear_field(matrix: &mut [Cell]) {
    for i in 0..matrix.len() {
        matrix[i] = Cell::Empty;
    }
}

pub fn flush(matrix: &[Cell], width: usize, out: &mut Stdout) {
    execute!(out, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0)).unwrap();

    let mut res: String = String::new();

    for row in matrix.chunks(width) {
        for cell in row {
            let val = decide_cell(&cell);
            res.push(val);
        }
        res.push('\n');
    }

    print!("{}", res);
}

pub fn decide_cell(cell: &Cell) -> char {
    return match cell {
        Cell::Empty => '-',
        Cell::Apple => '@',
        Cell::Snake => '#'
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty,
    Apple,
    Snake
}
