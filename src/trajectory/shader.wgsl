struct Output{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(@location(0) pos: vec3<f32>, @location(1) color: vec3<f32>) -> Output {
    var output: Output;
    output.pos = vec4(pos.xyz, 1.0);
    output.color = vec4(color.xyz, 1.0);
    return output;
}

@fragment
fn fs_main(@location(0) vColor: vec4<f32>) -> @location(0) vec4<f32> {
    return vColor;
}