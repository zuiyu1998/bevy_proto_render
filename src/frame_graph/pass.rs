use crate::error::RendererError;

use super::render_context::RenderContext;

pub trait PassData: Send + Sync + 'static {
    fn execute(&self, render_context: &mut RenderContext) -> Result<(), RendererError>;
}

pub type DynPass = Box<dyn PassData>;
