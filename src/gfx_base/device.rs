use crate::define_gfx_type;
use std::fmt::Debug;

use bevy::ecs::resource::Resource;
use downcast_rs::Downcast;

use super::{CommandBuffer, RenderPass, RenderPassInfo};

pub trait DeviceTrait: 'static + Sync + Send + Debug {
    fn create_command_buffer(&self) -> CommandBuffer;

    fn create_render_pass(&self, desc: &RenderPassInfo) -> RenderPass;

    fn submit(&self, command_buffers: Vec<CommandBuffer>);
}

pub trait ErasedDeviceTrait: 'static + Sync + Send + Downcast + Debug {
    fn create_command_buffer(&self) -> CommandBuffer;

    fn create_render_pass(&self, desc: &RenderPassInfo) -> RenderPass;
}

impl<T: DeviceTrait> ErasedDeviceTrait for T {
    fn create_command_buffer(&self) -> CommandBuffer {
        <T as DeviceTrait>::create_command_buffer(self)
    }

    fn create_render_pass(&self, desc: &RenderPassInfo) -> RenderPass {
        <T as DeviceTrait>::create_render_pass(self, desc)
    }
}

define_gfx_type!(Device, DeviceTrait, ErasedDeviceTrait);

impl Resource for Device {}

impl Device {
    pub fn create_command_buffer(&self) -> CommandBuffer {
        self.value.create_command_buffer()
    }

    pub fn create_render_pass(&self, desc: &RenderPassInfo) -> RenderPass {
        self.value.create_render_pass(desc)
    }
}
