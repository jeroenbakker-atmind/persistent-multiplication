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
    uint offset1;
    uint offset2;
    uint offset3;
    uint end;
    uint end1;
    uint end2;
    uint end3;
};


uint perform_step(uint idx) {
    uint result = 1;
    while (idx != 0) {
        result *= idx % 10;
        idx /= 10;
    }
    return result;
}

uint multiplication_persistent_32(uint idx) {
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
    uint persistent = multiplication_persistent_32(idx);
    buf.data[persistent] = 1;
}"
    }
}

pub mod compute_64 {
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
    uint offset1;
    uint offset2;
    uint offset3;
    uint end;
    uint end1;
    uint end2;
    uint end3;
};

uvec2 multiply(uvec2 a, uint b) {
    if (b == 0) {
        return uvec2(0);
    }
    if (b == 1) {
        return a;
    }

    uint rest = 0;
    uvec2 result = uvec2(0);
    umulExtended(a.y, b, result.x, result.y);
    result.x *= b;
    return result;
}

uvec2 perform_step(uvec2 idx) {
    uvec2 result = uvec2(0,1);
    while(idx != uvec2(0)) {
        result = multiply(result, idx.y %10);
        idx.y = idx.y / 10 + (idx.x % 10);
        idx.x = idx.x / 10;
    }
    return result;
}

uint multiplication_persistent_64(uvec2 idx) {
    uint result = 0;
    uvec2 new_idx = idx;
    while (new_idx.x != 0 || new_idx.y > 9) {
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
    uint persistent = multiplication_persistent_64(uvec2(0, idx));
    buf.data[persistent] = 1;
}"
    }
}
