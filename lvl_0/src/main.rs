// #![allow(unused)] // FIXME

use macroquad::prelude::*;

mod settings;
use settings::*;

mod assets;
mod command;
mod scene;
mod units;
mod utils;

use crate::scene::Scene;

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        scene.update();
        clear_background(GROUND_COLOR);
        scene.draw();
        next_frame().await
    }
}
