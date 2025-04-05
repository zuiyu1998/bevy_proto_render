mod error;
mod frame_graph;
mod gfx_base;
mod gfx_wgpu;
mod setup_pass;
mod setup_resource;

use std::sync::Arc;

pub use error::*;
pub use frame_graph::*;
pub use gfx_base::*;
pub use gfx_wgpu::*;
pub use setup_pass::*;
pub use setup_resource::*;

use bevy::{
    app::{App, Plugin},
    ecs::{
        entity::Entity,
        query::{With, Without},
        resource::Resource,
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut},
    },
    render::{
        Render, RenderApp, RenderSet,
        render_resource::PipelineCache,
        renderer::{RenderDevice as BevyRenderDevice, RenderQueue},
        view::ViewTarget,
    },
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum FrameGraphSet {
    ManageViews,
    SetupResource,
    SetupPass,
    SetupPassData,
    Compile,
    Execute,
}

#[derive(Debug, Resource)]
pub struct ProtoRenderDevice {
    pub device: Arc<Device>,
}

pub struct ProtoRenderPlugin;

impl Plugin for ProtoRenderPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_plugins(ImportedPassResourcePlugin::<ExtractedWindows>::default());
    }

    fn finish(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.configure_sets(
                Render,
                (
                    FrameGraphSet::ManageViews,
                    FrameGraphSet::SetupResource,
                    FrameGraphSet::SetupPass,
                    FrameGraphSet::SetupPassData,
                    FrameGraphSet::Compile,
                    FrameGraphSet::Execute,
                )
                    .chain()
                    .after(RenderSet::PrepareBindGroups)
                    .before(RenderSet::Render),
            );

            let bevy_render_device = render_app.world().resource::<BevyRenderDevice>().clone();
            let queue = render_app.world().resource::<RenderQueue>().clone();
            let render_device = Device::new(WgpuDevice {
                device: bevy_render_device,
                queue,
            });

            let mut setup_resources = SetupResources::default();

            setup_resources.add_node(SwapChainSetupResourceNode);

            render_app.insert_resource(ProtoRenderDevice {
                device: Arc::new(render_device),
            });
            render_app.insert_resource(TransientResourceCache::default());
            render_app.insert_resource(SetupPassesFrameGraph::default());
            render_app.insert_resource(setup_resources);

            render_app.add_systems(Render, init_frame_graph.in_set(FrameGraphSet::ManageViews));
            render_app.add_systems(
                Render,
                setup_resource_system.in_set(FrameGraphSet::SetupResource),
            );

            render_app.add_systems(Render, setup_pass_system.in_set(FrameGraphSet::SetupPass));

            render_app.add_systems(Render, compile_frame_graph.in_set(FrameGraphSet::Compile));
            render_app.add_systems(Render, execute_frame_graph.in_set(FrameGraphSet::Execute));
        }
    }
}

pub fn compile_frame_graph(mut frame_graphs: Query<&mut FrameGraph>) {
    for mut frame_graph in frame_graphs.iter_mut() {
        frame_graph.compile();
    }
}

pub fn init_frame_graph(
    mut commands: Commands,
    view_targets: Query<Entity, (Without<FrameGraph>, With<ViewTarget>)>,
    mut setup_passed: ResMut<SetupPassesFrameGraph>,
) {
    for view_target in view_targets.iter() {
        commands.entity(view_target).insert(FrameGraph::default());
        setup_passed.insert(view_target, SetupPasses::default());
    }
}

pub fn execute_frame_graph(
    render_device: Res<ProtoRenderDevice>,
    mut frame_graphs: Query<&mut FrameGraph>,
    mut transient_resource_cache: ResMut<TransientResourceCache>,
    pipeline_cache: Res<PipelineCache>,
    //mut windows: ResMut<ExtractedWindows>,
) {
    for mut frame_graph in frame_graphs.iter_mut() {
        frame_graph.execute(
            &render_device.device,
            &mut transient_resource_cache,
            &pipeline_cache,
        );
    }

    // for window in windows.values_mut() {
    //     if let Some(surface_texture) = window.swap_chain_texture.take() {
    //         // TODO(clean): winit docs recommends calling pre_present_notify before this.
    //         // though `present()` doesn't present the frame, it schedules it to be presented
    //         // by wgpu.
    //         // https://docs.rs/winit/0.29.9/wasm32-unknown-unknown/winit/window/struct.Window.html#method.pre_present_notify
    //         surface_texture.present();
    //     }
    // }
}

mod test {
    #[test]
    fn test() {
        let a = 5;

        assert_eq!(a, 5)
    }
}
