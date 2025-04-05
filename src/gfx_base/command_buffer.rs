use std::{fmt::Debug, ops::Range};

use bevy::render::render_resource::RenderPipeline;
use downcast_rs::Downcast;

use crate::define_gfx_type;

use super::{Device, RenderPass};

pub trait CommandBufferTrait: 'static + Sync + Send + Debug {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass);

    fn end_render_pass(&mut self);

    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
}

pub trait ErasedCommandBufferTrait: 'static + Sync + Send + Debug + Downcast {
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass);

    fn end_render_pass(&mut self);

    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
}

impl<T> ErasedCommandBufferTrait for T
where
    T: CommandBufferTrait,
{
    fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass) {
        <T as CommandBufferTrait>::begin_render_pass(self, device, render_pass);
    }

    fn end_render_pass(&mut self) {
        <T as CommandBufferTrait>::end_render_pass(self);
    }

    fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline) {
        <T as CommandBufferTrait>::set_render_pipeline(self, render_pipeline);
    }

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        <T as CommandBufferTrait>::draw(self, vertices, instances);
    }
}

define_gfx_type!(CommandBuffer, CommandBufferTrait, ErasedCommandBufferTrait);

impl CommandBuffer {
    pub fn begin_render_pass(&mut self, device: &Device, render_pass: RenderPass) {
        self.value.begin_render_pass(device, render_pass);
    }

    pub fn end_render_pass(&mut self) {
        self.value.end_render_pass();
    }

    pub fn set_render_pipeline(&mut self, render_pipeline: &RenderPipeline) {
        self.value.set_render_pipeline(render_pipeline);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.value.draw(vertices, instances);
    }
}
