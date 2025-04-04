use std::sync::Arc;

use crate::ColorAttachment;

use super::{
    DynPass, ImportToFrameGraph,
    graph::FrameGraph,
    pass_node::{GpuRead, GpuWrite, PassNode, ResourceNodeRef},
    resource::{FGResource, FGResourceDescriptor, TypeEquals},
    resource_node::ResourceNodeHandle,
};

pub struct PassNodeBuilder<'a> {
    graph: &'a mut FrameGraph,
    pass_node: Option<PassNode>,
}

impl Drop for PassNodeBuilder<'_> {
    fn drop(&mut self) {
        self.build();
    }
}

impl<'a> PassNodeBuilder<'a> {
    pub fn new(insert_point: usize, name: &str, graph: &'a mut FrameGraph) -> Self {
        let handle = graph.get_current_pass_node_handle();
        Self {
            graph,
            pass_node: Some(PassNode::new(insert_point, name, handle)),
        }
    }

    pub fn add_attachment(&mut self, color_attachment: ColorAttachment) {
        self.pass_node
            .as_mut()
            .unwrap()
            .add_attachment(color_attachment);
    }

    pub fn set_pass(&mut self, pass: DynPass) {
        self.pass_node.as_mut().unwrap().pass = Some(pass);
    }

    fn build(&mut self) {
        let pass_node = self.pass_node.take().unwrap();
        self.graph.add_pass_node(pass_node);
    }

    pub fn create<DescriptorType>(
        &mut self,
        name: &str,
        desc: DescriptorType,
    ) -> ResourceNodeHandle<DescriptorType::Resource>
    where
    DescriptorType: FGResourceDescriptor + TypeEquals<Other = <<DescriptorType as FGResourceDescriptor>::Resource as FGResource>::Descriptor>,
    {
        self.graph.create(name, desc)
    }

    pub fn import<ResourceType>(
        &mut self,
        name: &str,
        resource: Arc<ResourceType>,
        desc: ResourceType::Descriptor,
    ) -> ResourceNodeHandle<ResourceType>
    where
        ResourceType: ImportToFrameGraph,
    {
        self.graph.import(name, resource, desc)
    }

    pub fn write<ResourceType>(
        &mut self,
        resource_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuWrite> {
        self.pass_node
            .as_mut()
            .unwrap()
            .write(self.graph, resource_handle)
    }

    pub fn read<ResourceType>(
        &mut self,
        resource_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuRead> {
        self.pass_node
            .as_mut()
            .unwrap()
            .read(self.graph, resource_handle)
    }

    pub fn read_from_board<ResourceType>(
        &self,
        name: &str,
    ) -> Option<ResourceNodeHandle<ResourceType>> {
        self.graph
            .get_resource_board()
            .get(name)
            .map(|raw| ResourceNodeHandle::new(raw.resource_node_handle(), raw.resource_handle()))
    }
}
