use std::marker::PhantomData;

use super::{VirtualResource, handle::TypeHandle, pass_node::PassNode};

#[derive(Debug)]
pub struct ResourceNodeHandle<ResourceType> {
    handle: TypeHandle<ResourceNode>,
    resource_handle: TypeHandle<VirtualResource>,
    _marker: PhantomData<ResourceType>,
}

pub struct RawResourceNodeHandle {
    handle: TypeHandle<ResourceNode>,
    resource_handle: TypeHandle<VirtualResource>,
}

impl RawResourceNodeHandle {
    pub fn resource_handle(&self) -> TypeHandle<VirtualResource> {
        self.resource_handle
    }

    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.handle
    }
}

impl<ResourceType> ResourceNodeHandle<ResourceType> {
    pub fn new(
        handle: TypeHandle<ResourceNode>,
        resource_handle: TypeHandle<VirtualResource>,
    ) -> Self {
        ResourceNodeHandle {
            handle,
            resource_handle,
            _marker: PhantomData,
        }
    }

    pub fn raw(&self) -> RawResourceNodeHandle {
        RawResourceNodeHandle {
            handle: self.handle,
            resource_handle: self.resource_handle,
        }
    }

    pub fn resource_handle(&self) -> TypeHandle<VirtualResource> {
        self.resource_handle
    }

    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.handle
    }
}

pub struct ResourceNode {
    handle: TypeHandle<ResourceNode>,
    resource_handle: TypeHandle<VirtualResource>,
    pub version: u32,
    pub writer_handle: Option<TypeHandle<PassNode>>,
}

impl ResourceNode {
    pub fn new(
        handle: TypeHandle<ResourceNode>,
        resource_handle: TypeHandle<VirtualResource>,
        version: u32,
    ) -> Self {
        Self {
            handle,
            resource_handle,
            version,
            writer_handle: None,
        }
    }

    pub fn resource_node_handle(&self) -> TypeHandle<ResourceNode> {
        self.handle
    }

    pub fn resource_handle(&self) -> TypeHandle<VirtualResource> {
        self.resource_handle
    }
}
