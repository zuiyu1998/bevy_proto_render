use bevy::ecs::{resource::Resource, world::World};

use crate::error::RendererError;

use super::{pass_node_builder::PassNodeBuilder, render_context::RenderContext};

pub trait Pass: Send + Sync + 'static + Resource {
    type Data: PassData + Default;

    fn do_init(data: &mut Self::Data, world: &World);

    fn get_name(&self) -> &str;

    fn get_insert_point(&self) -> usize;
}

pub trait PassData: Send + Sync + 'static {
    ///构建frame graph
    fn setup(&mut self, builder: &mut PassNodeBuilder);

    fn execute(&self, render_context: &mut RenderContext) -> Result<(), RendererError>;
}

pub type DynPass = Box<dyn PassData>;
