
pub mod line;

pub trait Object {
    fn render(&self);
    fn selection(&self, x: f32, y: f32) -> bool;
}
