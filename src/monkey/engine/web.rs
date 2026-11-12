use crate::monkey::model::{Axis, Button, Coordinate, Direction, InputResult, Key};

use super::{Agent, Keyboard, Mouse};

pub struct BrowserMonkeyEngine {}

impl Mouse for BrowserMonkeyEngine {
    fn button(&mut self, _button: Button, _direction: Direction) -> InputResult<()> {
        todo!()
    }

    fn move_mouse(&mut self, _x: i32, _y: i32, _coordinate: Coordinate) -> InputResult<()> {
        todo!()
    }

    fn scroll(&mut self, _length: i32, _axis: Axis) -> InputResult<()> {
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
    fn fast_text(&mut self, _text: &str) -> InputResult<Option<()>> {
        todo!()
    }

    fn key(&mut self, _key: Key, _direction: Direction) -> InputResult<()> {
        todo!()
    }

    fn raw(&mut self, _keycode: u16, _direction: Direction) -> InputResult<()> {
        todo!()
    }
}

impl Agent for BrowserMonkeyEngine {}
