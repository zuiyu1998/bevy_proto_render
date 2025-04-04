use crate::define_gfx_type;
use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::frame_graph::RenderContext;

use super::ColorAttachment;

#[derive(Default, Clone, Debug)]
pub struct RenderPassInfo {
    pub color_attachments: Vec<ColorAttachment>,
}

impl RenderPassInfo {
    pub fn new() -> Self {
        RenderPassInfo::default()
    }
}

pub trait RenderPassTrait: 'static + Debug {
    fn do_init(&mut self, render_context: &RenderContext);
}

pub trait ErasedRenderPassTrait: 'static + Debug + Downcast {
    fn do_init(&mut self, render_context: &RenderContext);
}

impl<T: RenderPassTrait> ErasedRenderPassTrait for T {
    fn do_init(&mut self, render_context: &RenderContext) {
        <T as RenderPassTrait>::do_init(self, render_context);
    }
}

define_gfx_type!(RenderPass, RenderPassTrait, ErasedRenderPassTrait);

impl RenderPass {
    pub fn do_init(&mut self, render_context: &RenderContext) {
        self.value.do_init(render_context);
    }
}
