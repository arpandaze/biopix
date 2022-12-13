#[derive(Debug)]
pub struct Scene {
    pub vertices: Vec<f32>,
    pub normal_vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            normal_vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl Scene {
    fn add(&mut self, object: &mut impl crate::object::Object) {
        let old_vertex_count = self.vertices.len();

        self.vertices.append(&mut object.vertices());
        self.normal_vertices.append(&mut object.normal_vertices());

        let mut new_indices = object
            .indices()
            .iter()
            .map(|index| index + old_vertex_count as u32)
            .collect::<Vec<u32>>();

        self.indices.append(&mut new_indices);
    }
}
