use bevy::{
    ecs::{
        entity::EntityHashMap,
        resource::Resource,
        world::{EntityWorldMut, Mut, World},
    },
    platform_support::collections::HashMap,
    prelude::{Deref, DerefMut},
};
use downcast_rs::Downcast;

use crate::{
    ColorAttachment, FrameGraph, PassData, RenderContext, SetupResourceNode, SwapChain,
    SwapChainSetupResourceNode,
};

pub trait SetupPassNode: Downcast + Send + Sync + 'static {
    fn get_insert_point(&self) -> usize;
    fn get_pass_name() -> &'static str;

    fn setup_pass(&self, insert_point: usize, pass_name: &str, world: &mut EntityWorldMut<'_>);
}

pub struct SetupPassNodeState {
    node: Box<dyn ErasedSetupPassNode>,
    label: String,
    insert_point: usize,
}

pub trait ErasedSetupPassNode: Downcast + Send + Sync + 'static {
    fn get_insert_point(&self) -> usize;

    fn setup_pass(&self, insert_point: usize, pass_name: &str, world: &mut EntityWorldMut<'_>);
}

impl<T: SetupPassNode> ErasedSetupPassNode for T {
    fn setup_pass(&self, insert_point: usize, pass_name: &str, world: &mut EntityWorldMut) {
        <T as SetupPassNode>::setup_pass(self, insert_point, pass_name, world);
    }
    fn get_insert_point(&self) -> usize {
        <T as SetupPassNode>::get_insert_point(self)
    }
}

#[derive(Resource)]
pub struct SetupPasses {
    nodes: HashMap<String, SetupPassNodeState>,
    camera_deriver: SetupPassNodeState,
}

impl Default for SetupPasses {
    fn default() -> Self {
        let label = CameraDriverSetupPassNode::get_pass_name();
        let value = CameraDriverSetupPassNode::default();
        let insert_point = <CameraDriverSetupPassNode as SetupPassNode>::get_insert_point(&value);

        Self {
            nodes: Default::default(),
            camera_deriver: SetupPassNodeState {
                node: Box::new(value),
                label: label.to_string(),
                insert_point,
            },
        }
    }
}

impl SetupPasses {
    pub fn add_node<T: SetupPassNode>(&mut self, value: T) {
        let label = T::get_pass_name().to_string();
        let insert_point = value.get_insert_point();
        let node_state = SetupPassNodeState {
            node: Box::new(value),
            label: label.clone(),
            insert_point,
        };

        self.nodes.insert(label, node_state);
    }

    pub fn update(&mut self, world: &mut EntityWorldMut) {
        if self.nodes.is_empty() {
            self.camera_deriver.node.setup_pass(
                self.camera_deriver.insert_point,
                &self.camera_deriver.label,
                world,
            );
        }

        for node_state in self.nodes.values_mut() {
            node_state
                .node
                .setup_pass(node_state.insert_point, &node_state.label, world);
        }
    }
}

#[derive(Default, Resource, Deref, DerefMut)]
pub struct SetupPassesFrameGraph(EntityHashMap<SetupPasses>);

pub fn setup_pass_system(world: &mut World) {
    world.resource_scope(|world, mut setup_resources: Mut<SetupPassesFrameGraph>| {
        for (entity, setup_passes) in setup_resources.0.iter_mut() {
            let mut world = world.entity_mut(*entity);
            setup_passes.update(&mut world);
        }
    });
}

#[derive(Default)]
pub struct CameraDriverSetupPassNode {
    insert_point: usize,
}

impl SetupPassNode for CameraDriverSetupPassNode {
    fn get_pass_name() -> &'static str {
        "no_camera_clear_pass"
    }

    fn get_insert_point(&self) -> usize {
        self.insert_point
    }

    fn setup_pass(&self, insert_point: usize, pass_name: &str, world: &mut EntityWorldMut<'_>) {
        if let Some(mut frame_graph) = world.get_mut::<FrameGraph>() {
            let mut builder = frame_graph.create_pass_node_builder(insert_point, pass_name);

            if let Some(swap_chain_handle) = builder
                .read_from_board::<SwapChain>(SwapChainSetupResourceNode::get_resource_name())
            {
                let swap_chain_handle_read = builder.read(swap_chain_handle);
                builder.add_attachment(ColorAttachment::SwapChain(swap_chain_handle_read));

                builder.set_pass(CameraDriverPassData {});
            }
        }
    }
}

pub struct CameraDriverPassData {}

impl PassData for CameraDriverPassData {
    fn execute(&self, _render_context: &mut RenderContext) -> Result<(), crate::RendererError> {
        Ok(())
    }
}
