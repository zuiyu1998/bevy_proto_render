use std::fmt::Debug;

use bevy::render::renderer::RenderDevice as BevyRenderDevice;

use crate::{CommandBuffer, DeviceTrait, RenderPass, RenderPassInfo};

use super::{WgpuCommandBuffer, WgpuRenderPass};

pub struct WgpuRenderDevice {
    pub device: BevyRenderDevice,
}

impl Debug for WgpuRenderDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WgpuRenderDevice")
            .field("device", &self.device.wgpu_device())
            .finish()
    }
}

impl DeviceTrait for WgpuRenderDevice {
    fn create_command_buffer(&self) -> crate::CommandBuffer {
        CommandBuffer::new(WgpuCommandBuffer::default())
    }

    fn create_render_pass(&self, desc: &RenderPassInfo) -> crate::RenderPass {
        RenderPass::new(WgpuRenderPass::new(desc.clone()))
    }

    fn submit(&self, _command_buffers: Vec<crate::CommandBuffer>) {}
}
