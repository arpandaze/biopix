use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

#[derive(Clone)]
pub struct Model {
    pub vertex: Vec<f32>,
    pub normal_vertex: Vec<f32>,
    pub index: Vec<u32>,
}

impl Model{
    pub fn translate(&mut self, x: f32, y: f32, z: f32){


    }
}

impl From<&str> for Model {
    fn from(model_name: &str) -> Self {
        let input = BufReader::new(File::open(model_name).unwrap());
        let model: Obj = load_obj(input).unwrap();

        let vertices = model
            .vertices
            .iter()
            .flat_map(|item| {
                let pos = item.position;

                return [pos[0], pos[1], pos[2], 1.0, 1.0, 0.0];
            })
            .collect::<Vec<f32>>();

        let normals = model
            .vertices
            .iter()
            .flat_map(|item| {
                let pos = item.normal;

                return [pos[0], pos[1], pos[2], 1.0, 1.0, 0.0];
            })
            .collect::<Vec<f32>>();

        let indices = model
            .indices
            .iter()
            .map(|item| *item as u32)
            .collect::<Vec<u32>>();

        return Self {
            vertex: vertices,
            normal_vertex: normals,
            index: indices,
        };
    }
}
