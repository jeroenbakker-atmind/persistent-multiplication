extern crate vulkano;
extern crate vulkano_shaders;

mod rust_native;
mod vulkan_compute;
mod vulkan_shaders;

fn main() {
    let base = u32::MAX as u128 / 64;
    for i in 0..u32::MAX {
        let from = base * i as u128;
        let to = from + base;
        let result = vulkan_compute::calc_range(from, to);
        if result >= 10 {
            println!("{from}-{to} = {result}");
        }
    }
}
