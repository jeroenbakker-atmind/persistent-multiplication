use vulkano::buffer::*;
use vulkano::command_buffer::*;
use vulkano::descriptor_set::*;
use vulkano::device::physical::*;
use vulkano::device::*;
use vulkano::instance::*;
use vulkano::pipeline::layout::PipelineLayoutCreateInfo;
use vulkano::pipeline::layout::PushConstantRange;
use vulkano::pipeline::*;
use vulkano::shader::*;
use vulkano::sync::*;

use crate::vulkan_shaders;

pub fn calc_range(from: u32, to: u32) -> u8 {
    let instance = Instance::new(InstanceCreateInfo::default()).expect("failed to create instance");
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_compute())
        .expect("couldn't find a compute queue family");
    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            // here we pass the desired queue families that we want to use
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .expect("failed to create device");
    let queue = queues.next().unwrap();
    let data = [0; 256];
    let output_buffer =
        CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data)
            .expect("failed to create buffer");
    let shader =
        vulkan_shaders::compute_32::load(device.clone()).expect("failed to create shader module");

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        shader.entry_point("main").unwrap(),
        &(),
        None,
        |_| {},
    )
    .expect("failed to create compute pipeline");

    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        layout.clone(),
        [WriteDescriptorSet::buffer(0, output_buffer.clone())],
    )
    .unwrap();
    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let pushconstants_info = PipelineLayoutCreateInfo {
        push_constant_ranges: vec![PushConstantRange {
            stages: ShaderStages::compute(),
            offset: 0,
            size: 8,
        }],
        ..PipelineLayoutCreateInfo::default()
    };
    let pushconstants_layout = PipelineLayout::new(device.clone(), pushconstants_info).unwrap();

    builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0, // 0 is the index of our set
            set,
        )
        .fill_buffer(output_buffer.clone(), 0)
        .unwrap();
    let mut offset = from;

    while offset < to {
        builder
            .push_constants(pushconstants_layout.clone(), 0, offset)
            .push_constants(pushconstants_layout.clone(), 4, to)
            .dispatch([65535, 1, 1])
            .unwrap();
        offset += 1024 * 65536
    }

    let command_buffer = builder.build().unwrap();
    let future = now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let content = output_buffer.read().unwrap();
    for n in 0..256 {
        let value = content[255 - n];
        if value != 0 {
            return 255 - n as u8;
        }
    }
    return 0;
}

#[test]
fn persistent_step_0() {
    assert_eq!(calc_range(0, 1), 0);
}

#[test]
fn persistent_step_1() {
    assert_eq!(calc_range(0, 9), 0);
    assert_eq!(calc_range(0, 10), 1);
}

#[test]
fn persistent_step_2() {
    assert_eq!(calc_range(0, 24), 1);
    assert_eq!(calc_range(0, 25), 2);
}

#[test]
fn persistent_step_3() {
    assert_eq!(calc_range(0, 38), 2);
    assert_eq!(calc_range(0, 39), 3);
}

#[test]
fn persistent_step_4() {
    assert_eq!(calc_range(0, 76), 3);
    assert_eq!(calc_range(0, 77), 4);
}

#[test]
fn persistent_step_5() {
    assert_eq!(calc_range(0, 678), 4);
    assert_eq!(calc_range(0, 679), 5);
}

#[test]
fn persistent_step_6() {
    assert_eq!(calc_range(0, 6787), 5);
    assert_eq!(calc_range(0, 6788), 6);
}

#[test]
fn persistent_step_7() {
    assert_eq!(calc_range(0, 68888), 6);
    assert_eq!(calc_range(0, 68889), 7);
}

#[test]
fn persistent_step_8() {
    assert_eq!(calc_range(0, 2677888), 7);
    assert_eq!(calc_range(0, 2677889), 8);
}

#[test]
fn persistent_step_9() {
    assert_eq!(calc_range(0, 26888998), 8);
    assert_eq!(calc_range(0, 26888999), 9);
}

#[test]
fn persistent_step_10() {
    assert_eq!(calc_range(0, 3778888998), 9);
    assert_eq!(calc_range(0, 3778888999), 10);
}
