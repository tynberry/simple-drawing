pub mod obj;

use macroquad::prelude::*;

use crate::obj::{Object, line::Line};

#[macroquad::main("Paint")]
async fn main() {

    let mut objects: Vec<Box<dyn Object>> = vec![];

    objects.push(Box::new(Line{
        first: Vec2::new(15.0,0.0),
        second: Vec2::new(69.0, 17.0),
        thickness: 7.0,
        color: RED,
    }));

    loop{
        clear_background(WHITE);

        //render objects 
        for obj in &objects {
            obj.render();
        }

        next_frame().await;
    }
}
