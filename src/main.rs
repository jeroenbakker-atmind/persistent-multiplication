
extern crate vulkano;
extern crate vulkano_shaders;

mod rust_native;
mod vulkan_compute;
mod vulkan_shaders;

fn main() {
     let result = vulkan_compute::calc_range(0,u32::MAX as u64);
    println!("result: {result}");
}
