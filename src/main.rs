mod cylinder;
mod object;
mod opengl;
mod scene;
mod sphere;

use lazy_static::lazy_static;

use opengl::*;

lazy_static! {
    static ref SCENE: scene::Scene = scene::Scene::from("1d66.pdb");
}

pub fn main() {
    opengl::init(None, &SCENE);
}
