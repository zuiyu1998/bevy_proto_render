use std::{ops::Deref, sync::Arc};

use bevy::{
    ecs::{
        resource::Resource,
        system::{Query, Res, SystemState},
        world::{Mut, World},
    },
    platform_support::collections::HashMap,
    render::{
        camera::{ExtractedCamera, ManualTextureViews},
        render_asset::RenderAssets,
        texture::GpuImage,
        view::ExtractedWindows,
    },
};
use downcast_rs::Downcast;

use crate::{FrameGraph, SwapChain, SwapChainInfo, WgpuSwapChain};

pub trait SetupResourceNode: Downcast + Send + Sync + 'static {
    fn get_resource_name() -> &'static str;

    fn setup_resource(&self, resource_name: &str, world: &mut World);
}

pub struct SetupResourceNodeState {
    node: Box<dyn ErasedSetupResourceNode>,
    label: String,
}

pub trait ErasedSetupResourceNode: Downcast + Send + Sync + 'static {
    fn setup_resource(&self, resource_name: &str, world: &mut World);
}

impl<T: SetupResourceNode> ErasedSetupResourceNode for T {
    fn setup_resource(&self, resource_name: &str, world: &mut World) {
        <T as SetupResourceNode>::setup_resource(self, resource_name, world);
    }
}

#[derive(Resource, Default)]
pub struct SetupResources {
    nodes: HashMap<String, SetupResourceNodeState>,
}

impl SetupResources {
    pub fn add_node<T: SetupResourceNode>(&mut self, value: T) {
        let label = T::get_resource_name().to_string();
        let node_state = SetupResourceNodeState {
            node: Box::new(value),
            label: label.clone(),
        };

        self.nodes.insert(label, node_state);
    }

    pub fn update(&mut self, world: &mut World) {
        for node_state in self.nodes.values_mut() {
            node_state.node.setup_resource(&node_state.label, world);
        }
    }
}

pub fn setup_resource_system(world: &mut World) {
    world.resource_scope(|world, mut setup_resources: Mut<SetupResources>| {
        setup_resources.update(world);
    });
}

pub struct SwapChainSetupResourceNode;

impl SetupResourceNode for SwapChainSetupResourceNode {
    fn get_resource_name() -> &'static str {
        "swap_chain"
    }

    fn setup_resource<'w>(&self, resource_name: &str, world: &mut World) {
        let mut state = SystemState::<(
            Query<(&ExtractedCamera, &mut FrameGraph)>,
            Res<ExtractedWindows>,
            Res<ManualTextureViews>,
            Res<RenderAssets<GpuImage>>,
        )>::new(world);

        let (mut frame_graphs, windows, manual_texture_views, images) = state.get_mut(world);

        for (camera, mut frame_graph) in frame_graphs.iter_mut() {
            if camera.target.is_none() {
                return;
            }

            if let Some(texture_view) = camera.target.as_ref().unwrap().get_texture_view(
                &windows,
                &images,
                &manual_texture_views,
            ) {
                let swap_chain: Arc<SwapChain> = Arc::new(SwapChain::new(WgpuSwapChain {
                    texture_view: texture_view.deref().clone(),
                }));

                frame_graph.import(
                    resource_name,
                    swap_chain,
                    SwapChainInfo {
                        name: resource_name.to_string(),
                    },
                );
            }
        }
    }
}
