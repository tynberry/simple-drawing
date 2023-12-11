use egui::DragValue;
use macroquad::prelude::*;

#[macroquad::main("egui with macroquad")]
async fn main() {
    //YES
    let mut s_1: f64 = 0.0;
    let mut s_2: f64 = 0.0;
    let mut ang: f64 = 0.0;

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Ovládání")
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Strana 1");
                        ui.add(egui::widgets::DragValue::new(&mut s_1));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Strana 2");
                        ui.add(egui::widgets::DragValue::new(&mut s_2));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Uhel");

                        let mut degrees = ang.to_degrees();
                        let mut response = ui.add(DragValue::new(&mut degrees).speed(1.0).suffix("°"));

                        // only touch `*radians` if we actually changed the degree value
                        if degrees != ang.to_degrees() {
                            ang = degrees.to_radians();
                            response.changed = true;
                        }

                    });
                });
        });

        egui_macroquad::draw();

        //vypočti třetí stranu 
        let s_3 = s_1.powi(2) + s_2.powi(2) + s_1 * s_2 * ang.cos();

        //vykresli trojúhelník
        

        next_frame().await;
    }
}
