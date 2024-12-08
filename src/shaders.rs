pub mod default_vertex_shader {
    use vulkano_shaders::shader;
    shader!(ty: "vertex", path: "src/shaders/default_vertex.vert");   
}

pub mod default_fragment_shader {
    use vulkano_shaders::shader;
    shader!(ty: "fragment", path: "src/shaders/default_fragment.frag");
}