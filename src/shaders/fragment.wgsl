struct VertexOutput {
    // builtin to telle wgpu that this variable will contain the vertex clip position
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // tells wgpu to store the value returned in the first color target
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}