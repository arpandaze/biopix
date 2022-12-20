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

        let scale = &pdb.scale;

        println!("{:?}", scale);

        let mut spheres = Vec::<Sphere>::new();
        atoms.iter().for_each(|atom| {
            if !atom.hetero() {
                match atom.element() {
                    Some(Element::O) => {
                        let mut model = Sphere::new(10, 10, 100.0, [1.0, 0.0, 1.0]);
                        model.scale(0.012369, 0.012369, 0.012369);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        spheres.push(model);
                    }
                    Some(Element::H) => {
                        let mut model = Sphere::new(10, 10, 100.0, [1.0, 0.0, 0.0]);
                        model.scale(0.012369, 0.012369, 0.012369);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        spheres.push(model);
                    }
                    Some(Element::C) => {
                        let mut model = Sphere::new(10, 10, 100.0, [0.0, 1.0, 0.0]);
                        model.scale(0.012369, 0.012369, 0.012369);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        spheres.push(model);
                    }
                    Some(Element::N) => {
                        let mut model = Sphere::new(10, 10, 100.0, [0.0, 0.0, 1.0]);
                        model.scale(0.012369, 0.012369, 0.012369);
                        model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                        spheres.push(model);
                    }
                    _ => {}
                }
            }
        });

        return Self {
            spheres: Vec::from(spheres),
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
