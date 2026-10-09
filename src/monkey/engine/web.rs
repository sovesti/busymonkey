use crate::monkey::model::{Axis, Button, Coordinate, Direction, InputResult, Key};

use super::{Agent, Keyboard, Mouse};

pub struct BrowserMonkeyEngine {}

impl Mouse for BrowserMonkeyEngine {
    fn button(&mut self, button: Button, direction: Direction) -> InputResult<()> {
        todo!()
    }

    fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) -> InputResult<()> {
        todo!()
    }

    fn scroll(&mut self, length: i32, axis: Axis) -> InputResult<()> {
        todo!()
    }

    fn main_display(&self) -> InputResult<(i32, i32)> {
        todo!()
    }

    fn location(&self) -> InputResult<(i32, i32)> {
        todo!()
    }
}

impl Keyboard for BrowserMonkeyEngine {
    fn fast_text(&mut self, text: &str) -> InputResult<Option<()>> {
        todo!()
    }

    fn key(&mut self, key: Key, direction: Direction) -> InputResult<()> {
        todo!()
    }

    fn raw(&mut self, keycode: u16, direction: Direction) -> InputResult<()> {
        todo!()
    }
}

impl Agent for BrowserMonkeyEngine {}
