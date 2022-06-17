
pub mod compute_32 {
    use vulkano_shaders;

    vulkano_shaders::shader! {
        ty: "compute",
        src: "
#version 450

layout(local_size_x = 1024, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

layout(push_constant) uniform my_push_constants
{
    uint offset;
    uint end;
};


uint perform_step(uint idx) {
    uint result = 1;
    while (idx != 0) {
        result *= idx % 10;
        idx /= 10;
    }
    return result;
}

uint multiplication_persistent(uint idx) {
    uint result = 0;
    uint new_idx = idx;
    while (new_idx > 9) {
        new_idx = perform_step(new_idx);
        result += 1;
    }
    return result;
}

void main() {
    uint idx = gl_GlobalInvocationID.x + 1 + offset;
    if (idx > end) {
        return;
    }
    uint persistent = multiplication_persistent(idx);
    buf.data[persistent] = 1;
}"
    }
}
