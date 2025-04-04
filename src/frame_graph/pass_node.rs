use std::marker::PhantomData;

use crate::ColorAttachment;

use super::{
    DynPass,
    graph::FrameGraph,
    handle::TypeHandle,
    resource::VirtualResource,
    resource_node::{ResourceNode, ResourceNodeHandle},
};

pub struct PassNode {
    pub name: String,
    pub handle: TypeHandle<PassNode>,
    pub writes: Vec<TypeHandle<ResourceNode>>,
    pub reads: Vec<TypeHandle<ResourceNode>>,
    pub insert_point: usize,
    ///使用资源的获取生命周期
    pub resource_request_array: Vec<TypeHandle<VirtualResource>>,
    ///使用资源的释放生命周期
    pub resource_release_array: Vec<TypeHandle<VirtualResource>>,
    pub pass: Option<DynPass>,
    pub color_attachments: Vec<ColorAttachment>,
}

impl PassNode {
    pub fn new(insert_point: usize, name: &str, handle: TypeHandle<PassNode>) -> Self {
        PassNode {
            name: name.to_string(),
            handle,
            writes: vec![],
            reads: vec![],
            insert_point,
            resource_request_array: vec![],
            resource_release_array: vec![],
            pass: None,
            color_attachments: vec![],
        }
    }

    pub fn add_attachment(&mut self, color_attachment: ColorAttachment) {
        self.color_attachments.push(color_attachment);
    }

    pub fn write<ResourceType>(
        &mut self,
        graph: &mut FrameGraph,
        resource_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuWrite> {
        let resource_handle = graph
            .get_resource_node(&resource_handle.resource_node_handle())
            .resource_handle();

        let resource = graph.get_resource_mut(&resource_handle);
        resource.info.new_version();

        let resource_info = resource.info.clone();
        let new_resource_node_handle = graph.create_resource_node(resource_info);
        let new_resource_node = graph.get_resource_node_mut(&new_resource_node_handle);
        new_resource_node.writer_handle = Some(self.handle);

        self.writes.push(new_resource_node_handle);

        ResourceNodeRef::new(new_resource_node_handle, resource_handle)
    }

    pub fn read<ResourceType>(
        &mut self,
        graph: &mut FrameGraph,
        resource_handle: ResourceNodeHandle<ResourceType>,
    ) -> ResourceNodeRef<ResourceType, GpuRead> {
        let resource_node_handle = resource_handle.resource_node_handle();

        if !self.reads.contains(&resource_node_handle) {
            self.reads.push(resource_node_handle);
        }

        let resource_handle = graph
            .get_resource_node(&resource_node_handle)
            .resource_handle();

        ResourceNodeRef::new(resource_node_handle, resource_handle)
    }
}

#[derive(Debug)]
pub struct ResourceNodeRef<ResourceType, ViewType> {
    handle: TypeHandle<ResourceNode>,
    resource_handle: TypeHandle<VirtualResource>,
    _marker: PhantomData<(ResourceType, ViewType)>,
}

impl<ResourceType, ViewType> Clone for ResourceNodeRef<ResourceType, ViewType> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            resource_handle: self.resource_handle,
            _marker: PhantomData,
        }
    }
}

impl<ResourceType, ViewType> ResourceNodeRef<ResourceType, ViewType> {
    pub fn new(
        handle: TypeHandle<ResourceNode>,
        resource_handle: TypeHandle<VirtualResource>,
    ) -> Self {
        ResourceNodeRef {
            handle,
            resource_handle,
            _marker: PhantomData,
        }
    }

    pub fn resource_handle(&self) -> TypeHandle<VirtualResource> {
        self.resource_handle
    }
    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.handle
    }
}

pub trait GpuViewType: 'static {}

#[derive(Debug)]
pub struct GpuRead;

impl GpuViewType for GpuRead {}

pub struct GpuWrite;

impl GpuViewType for GpuWrite {}
