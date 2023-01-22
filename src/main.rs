mod cylinder;
mod object;
mod opengl;
mod scene;
mod sphere;

use opengl::*;
use std::env;

pub fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Missing file argument! Please pass filename as argument.");
    }

    let render_scene: scene::Scene = scene::Scene::from(args.get(1).unwrap());
    opengl::init(&render_scene);
}
