mod error;
mod frame_graph;
mod gfx_base;
mod gfx_wgpu;
mod render_resource;

use std::{marker::PhantomData, sync::Arc};

pub use error::*;
pub use frame_graph::*;
pub use gfx_base::*;
pub use gfx_wgpu::*;
pub use render_resource::*;

use bevy::{
    app::{App, Plugin},
    ecs::{
        resource::Resource,
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Res, ResMut, SystemState},
        world::World,
    },
    render::{
        Render, RenderApp, RenderSet, renderer::RenderDevice as BevyRenderDevice,
        view::ExtractedWindows,
    },
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum FrameGraphSet {
    SetupResource,
    SetupPass,
    Compile,
    Execute,
}

#[derive(Debug, Resource)]
pub struct ProtoRenderDevice {
    pub device: Arc<Device>,
}

#[derive(Default)]
pub struct ImportedPassResourcePlugin<R> {
    _marker: PhantomData<R>,
}

pub fn imported_pass_resource<R: ImportedPassResource>(
    resource: Res<R>,
    mut frame_graph: ResMut<FrameGraph>,
) {
    resource.imported(&mut frame_graph);
}

impl<R: ImportedPassResource> Plugin for ImportedPassResourcePlugin<R> {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(
                Render,
                imported_pass_resource::<R>.in_set(FrameGraphSet::SetupResource),
            );
        }
    }
}

#[derive(Default)]
pub struct PassPlugin<P> {
    _marker: PhantomData<P>,
}

pub fn pass_setup<P: Pass + Default>(world: &mut World) {
    let mut pass_data = P::Data::default();
    P::do_init(&mut pass_data, world);

    let mut state = SystemState::<(ResMut<P>, ResMut<FrameGraph>)>::new(world);
    let (pass, mut frame_graph) = state.get_mut(world);
    let mut builder =
        frame_graph.create_pass_node_builder(pass.get_insert_point(), pass.get_name());
    pass_data.setup(&mut builder);
    builder.set_pass(Box::new(pass_data));
}

impl<P: Pass + Default> Plugin for PassPlugin<P> {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.init_resource::<P>();

            render_app.add_systems(Render, pass_setup::<P>.in_set(FrameGraphSet::SetupPass));
        }
    }
}

pub struct ProtoRenderPlugin;

impl Plugin for ProtoRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ImportedPassResourcePlugin::<ExtractedWindows>::default());
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.configure_sets(
                Render,
                (
                    FrameGraphSet::SetupResource,
                    FrameGraphSet::SetupPass,
                    FrameGraphSet::Compile,
                    FrameGraphSet::Execute,
                )
                    .chain()
                    .after(RenderSet::Prepare)
                    .before(RenderSet::Render),
            );

            let bevy_render_device = render_app.world().resource::<BevyRenderDevice>().clone();
            let render_device = Device::new(WgpuRenderDevice {
                device: bevy_render_device,
            });

            render_app.insert_resource(ProtoRenderDevice {
                device: Arc::new(render_device),
            });
            render_app.insert_resource(FrameGraph::default());
            render_app.insert_resource(TransientResourceCache::default());

            render_app.add_systems(Render, compile_frame_graph.in_set(FrameGraphSet::Compile));
            render_app.add_systems(Render, execute_frame_graph.in_set(FrameGraphSet::Execute));
        }
    }
}

pub fn compile_frame_graph(mut frame_graph: ResMut<FrameGraph>) {
    frame_graph.compile();
}

pub fn execute_frame_graph(
    render_device: Res<ProtoRenderDevice>,
    mut frame_graph: ResMut<FrameGraph>,
    mut transient_resource_cache: ResMut<TransientResourceCache>,
) {
    frame_graph.execute(&render_device.device, &mut transient_resource_cache);

    frame_graph.reset();
}

mod test {
    #[test]
    fn test() {
        let a = 5;

        assert_eq!(a, 5)
    }
}
