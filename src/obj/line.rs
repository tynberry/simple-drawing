use macroquad::prelude::*;

use super::Object;


#[derive(Clone, Copy)]
pub struct Line {
    pub first: Vec2,
    pub second: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl Object for Line {
    fn render(&self) {
        draw_line(
            self.first.x, 
            self.first.y,
            self.second.x,
            self.second.y, 
            self.thickness, 
            self.color
        )
    }

    fn selection(&self, x: f32, y: f32) -> bool {
        todo!()
    }
}