use crate::cylinder::Cylinder;
use crate::object::Object;
use crate::sphere::Sphere;
use pdbtbx;
use pdbtbx::*;

#[derive(Clone)]
pub enum ModelTypes {
    Sphere(super::sphere::Sphere),
    Cylinder(super::cylinder::Cylinder),
}

#[derive(Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub cyliders: Vec<Cylinder>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            spheres: Vec::new(),
            cyliders: Vec::new(),
        }
    }
}

impl From<&str> for Scene {
    fn from(filename: &str) -> Self {
        let (mut pdb, _) = pdbtbx::open_pdb(filename, StrictnessLevel::Loose).unwrap();

        let atoms = pdb.atoms().collect::<Vec<&Atom>>();

        let scale_matrix = &pdb.scale.clone().unwrap().matrix();

        let scale_x = scale_matrix[0][0] as f32;
        let scale_y = scale_matrix[1][1] as f32;
        let scale_z = scale_matrix[2][2] as f32;

        let mut spheres = Vec::<Sphere>::new();
        let mut centre: [f32; 3] = [0.0, 0.0, 0.0];
        atoms.iter().for_each(|atom| {
            if !atom.hetero() {
                match atom.element() {
                    Some(Element::O) => {
                        let mut model = Sphere::new(10, 10, 25.0, [1.0, 0.0, 1.0]);
                        model.scale(scale_x, scale_y, scale_z);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        centre = [
                            centre[0] + atom.x() as f32,
                            centre[1] + atom.y() as f32,
                            centre[2] + atom.z() as f32,
                        ];
                        spheres.push(model);
                    }
                    Some(Element::H) => {
                        let mut model = Sphere::new(10, 10, 25.0, [1.0, 0.0, 0.0]);
                        model.scale(scale_x, scale_y, scale_z);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        centre = [
                            centre[0] + atom.x() as f32,
                            centre[1] + atom.y() as f32,
                            centre[2] + atom.z() as f32,
                        ];
                        spheres.push(model);
                    }
                    Some(Element::C) => {
                        let mut model = Sphere::new(10, 10, 50.0, [0.0, 1.0, 0.0]);
                        model.scale(scale_x, scale_y, scale_z);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        centre = [
                            centre[0] + atom.x() as f32,
                            centre[1] + atom.y() as f32,
                            centre[2] + atom.z() as f32,
                        ];
                        spheres.push(model);
                    }
                    Some(Element::N) => {
                        let mut model = Sphere::new(10, 10, 42.3, [0.0, 0.0, 1.0]);
                        model.scale(scale_x, scale_y, scale_z);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        spheres.push(model);
                    }
                    _ => {}
                }
            }
        });

        centre = [
            centre[0] / spheres.len() as f32,
            centre[1] / spheres.len() as f32,
            centre[2] / spheres.len() as f32,
        ];

        for atom in spheres.iter_mut() {
            atom.translate(-centre[0], -centre[1], -centre[2]);
        }


        return Self {
            spheres,
            cyliders: vec![],
        };
    }
}

impl Scene {
    pub fn add(&mut self, object: ModelTypes) {
        match object {
            ModelTypes::Sphere(s) => self.spheres.push(s),
            ModelTypes::Cylinder(c) => self.cyliders.push(c),
        }
    }

    pub fn render(&self, renderer: &mut crate::opengl::Renderer) {
        for model in &self.spheres {
            unsafe {
                model.drawer(renderer);
            }
        }
    }
}

#[test]
fn testpdb() {
    let test = Scene::from("1k8h.pdb");

    println!("{:?}", test.spheres.len());
}
