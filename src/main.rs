pub mod obj;

use core::str::next_code_point;

use macroquad::prelude::*;

use crate::obj::Object;

#[macroquad::main("Paint")]
async fn main() {

    let mut objects: Vec<Box<dyn Object>> = vec![];

    loop{
        clear_background(WHITE);

        //render objects 
        for obj in &objects {
            obj.render();
        }

        next_frame().await;
    }
}
