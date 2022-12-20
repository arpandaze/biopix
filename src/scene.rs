use crate::object::Object;

#[derive(Debug)]
pub struct Scene {
    pub vertices: Vec<f32>,
    pub normal_vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            normal_vertices: Vec::new(),
            indices: Vec::new(),
            colors: Vec::new(),
        }
    }
}

impl Scene {
    pub fn add(&mut self, object: &dyn Object) {
        let old_vertex_count = self.vertices.len();

        self.vertices.append(&mut object.vertices().clone());
        self.normal_vertices.append(&mut object.normal_vertices().clone());
        self.colors.append(&mut object.colors().clone());

        let mut new_indices = object
            .indices()
            .iter()
            .map(|index| index + old_vertex_count as u32)
            .collect::<Vec<u32>>();

        self.indices.append(&mut new_indices);
    }
}

impl Object for Scene {
    fn indices(&self) -> &Vec<u32> {
        return &self.indices;
    }

    fn vertices_mut(&mut self) -> &mut Vec<f32> {
        return &mut self.vertices;
    }

    fn vertices(&self) -> &Vec<f32> {
        return &self.vertices;
    }

    fn colors(&self) -> &Vec<f32> {
        return &self.colors;
    }

    fn normal_vertices(&self) -> &Vec<f32> {
        return &self.normal_vertices;
    }
}

#[test]
fn test_scene() {
    let sphere1 = crate::sphere::Sphere::new(3, 3, 5.0, [1.0, 1.0, 1.0]);
    let mut sphere2 = crate::sphere::Sphere::new(3, 3, 5.0, [1.0, 1.0, 1.0]);
    sphere2.translate(10.0, 0.0, 0.0);

    let mut test_scene = Scene::default();

    test_scene.add(&sphere2);
    test_scene.add(&sphere1);

    println!("{:?}",test_scene.interlaced_vertices());
}
