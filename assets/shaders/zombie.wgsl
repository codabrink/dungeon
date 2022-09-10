struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) world_position: vec4<f32>,
  @location(1) world_normal: vec3<f32>,
  @location(2) uv: vec2<f32>,
}

struct MyMat {
  color: vec4<f32>
}

@group(1) @binding(0)
var<uniform> color: MyMat;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
  return color.color;
}
