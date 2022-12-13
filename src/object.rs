pub trait Object {
    fn vertices(&mut self) -> &mut Vec<f32>;
    fn normal_vertices(&mut self) -> &mut Vec<f32>;
    fn indices(&mut self) -> &mut Vec<u32>;
}
