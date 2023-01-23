use crate::cylinder::Cylinder;
use crate::object::Object;
use crate::sphere::Sphere;
use pdbtbx;
use pdbtbx::*;

const SPHERE_SECTOR: u32 = 10;
const SPHERE_STACK: u32 = 10;

const CYLINDER_STACK: u32 = 10;

#[derive(Clone)]
pub enum ModelTypes {
    Sphere(super::sphere::Sphere),
    Cylinder(super::cylinder::Cylinder),
}

// #[derive(Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub cylinders: Vec<Cylinder>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            spheres: Vec::new(),
            cylinders: Vec::new(),
        }
    }
}

impl From<&String> for Scene {
    fn from(filename: &String) -> Self {
        let (mut pdb, _) = pdbtbx::open_pdb(filename, StrictnessLevel::Loose).unwrap();

        // let atoms = pdb.atoms().collect::<Vec<&Atom>>();

        let scale_matrix = &pdb.scale.clone().unwrap().matrix();

        let mut chains = pdb.chains_mut().collect::<Vec<&mut Chain>>();

        let scale_x = scale_matrix[0][0] as f32;
        let scale_y = scale_matrix[1][1] as f32;
        let scale_z = scale_matrix[2][2] as f32;

        let mut spheres = Vec::<Sphere>::new();
        let mut cylinders = Vec::<Cylinder>::new();
        let mut centre: [f32; 3] = [0.0, 0.0, 0.0];

        chains.iter_mut().for_each(|chain| {
            let mut chain_beginning = true;

            chain.sort();
            chain.atoms().for_each(|atom| {
                if !atom.hetero() {
                    let sphere_model = match atom.element() {
                        Some(Element::O) => {
                            let mut model =
                                Sphere::new(SPHERE_SECTOR, SPHERE_STACK, 25.0, [1.0, 0.0, 1.0]);
                            model.scale(scale_x, scale_y, scale_z);
                            model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                            centre = [
                                centre[0] + atom.x() as f32,
                                centre[1] + atom.y() as f32,
                                centre[2] + atom.z() as f32,
                            ];
                            Some(model)
                        }
                        Some(Element::H) => {
                            let mut model =
                                Sphere::new(SPHERE_SECTOR, SPHERE_STACK, 25.0, [1.0, 0.0, 0.0]);
                            model.scale(scale_x, scale_y, scale_z);
                            model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                            centre = [
                                centre[0] + atom.x() as f32,
                                centre[1] + atom.y() as f32,
                                centre[2] + atom.z() as f32,
                            ];
                            Some(model)
                        }
                        Some(Element::C) => {
                            let mut model =
                                Sphere::new(SPHERE_SECTOR, SPHERE_STACK, 50.0, [0.0, 1.0, 0.0]);
                            model.scale(scale_x, scale_y, scale_z);
                            model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                            centre = [
                                centre[0] + atom.x() as f32,
                                centre[1] + atom.y() as f32,
                                centre[2] + atom.z() as f32,
                            ];
                            Some(model)
                        }
                        Some(Element::N) => {
                            let mut model =
                                Sphere::new(SPHERE_SECTOR, SPHERE_STACK, 42.3, [0.0, 0.0, 1.0]);
                            model.scale(scale_x, scale_y, scale_z);
                            model.translate(atom.x() as f32, atom.y() as f32, atom.z() as f32);
                            Some(model)
                        }
                        _ => None,
                    };

                    if sphere_model.is_some() {
                        if !chain_beginning {
                            let mut cyl_model =
                                Cylinder::new(0.2, 1.0, CYLINDER_STACK, [1.0, 1.0, 1.0]);


                            cyl_model.align_axis(
                                spheres.last().unwrap().centre,
                                sphere_model.as_ref().unwrap().centre,
                            );

                            cylinders.push(cyl_model);
                        }

                        spheres.push(sphere_model.unwrap());
                    }

                    chain_beginning = false;
                }
            });
        });

        centre = [
            centre[0] / spheres.len() as f32,
            centre[1] / spheres.len() as f32,
            centre[2] / spheres.len() as f32,
        ];

        for atom in spheres.iter_mut() {
            atom.translate(-centre[0], -centre[1], -centre[2]);
        }

        for bond in cylinders.iter_mut() {
            bond.translate(-centre[0], -centre[1], -centre[2]);
        }

        return Self {
            spheres,
            cylinders,
        };
    }
}

impl Scene {
    pub fn add(&mut self, object: ModelTypes) {
        match object {
            ModelTypes::Sphere(s) => self.spheres.push(s),
            ModelTypes::Cylinder(c) => self.cylinders.push(c),
        }
    }

    pub fn render(&self, renderer: &mut crate::opengl::Renderer) {
        for model in &self.spheres {
            unsafe {
                model.drawer(renderer);
            }
        }

        for model in &self.cylinders {
            unsafe {
                model.drawer(renderer);
            }
        }
    }
}

#[test]
fn testpdb() {
    let test = Scene::from("1k8h.pdb".to_string());

    println!("{:?}", test.spheres.len());
}
