use std::fmt::Debug;

use bevy::render::renderer::{RenderDevice, RenderQueue};

use crate::{CommandBuffer, DeviceTrait, RenderPass, RenderPassInfo};

use super::{WgpuCommandBuffer, WgpuRenderPass};

pub struct WgpuDevice {
    pub device: RenderDevice,
    pub queue: RenderQueue,
}

impl Debug for WgpuDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WgpuRenderDevice")
            .field("device", &self.device.wgpu_device())
            .finish()
    }
}

impl DeviceTrait for WgpuDevice {
    fn create_command_buffer(&self) -> crate::CommandBuffer {
        CommandBuffer::new(WgpuCommandBuffer::default())
    }

    fn create_render_pass(&self, desc: &RenderPassInfo) -> crate::RenderPass {
        RenderPass::new(WgpuRenderPass::new(desc.clone()))
    }

    fn submit(&self, command_buffers: Vec<crate::CommandBuffer>) {
        let mut targets = vec![];

        for command_buffer in command_buffers.into_iter() {
            let mut command_buffer = command_buffer.downcast::<WgpuCommandBuffer>().unwrap();

            if let Some(command_buffer) = command_buffer.command_buffer.take() {
                targets.push(command_buffer);
            }
        }

        self.queue.submit(targets);
    }
}
