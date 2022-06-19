use crate::rust_native::MultiplicationSteps;

extern crate vulkano;
extern crate vulkano_shaders;

mod rust_native;
mod vulkan_compute;
mod vulkan_shaders;

fn main() {
     let result = vulkan_compute::calc_range(0,3778888999);
    //let result = 3778888999_u32.multiplication_steps(&10, &0);
    println!("result: {result}");
}
