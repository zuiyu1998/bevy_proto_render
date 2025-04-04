mod swap_chain;
mod texture;

use std::{fmt::Debug, sync::Arc};

use crate::{Device, SwapChain, SwapChainInfo, Texture, TextureInfo};

use super::{handle::TypeHandle, pass_node::PassNode};

impl Device {
    pub fn create(&self, _desc: &AnyFGResourceDescriptor) -> AnyFGResource {
        todo!()
    }
}

#[derive(Clone)]
pub struct VirtualResource {
    pub state: ResourceState,
    pub info: ResourceInfo,
}

impl VirtualResource {
    pub fn setup<ResourceType: FGResource>(
        name: &str,
        handle: TypeHandle<VirtualResource>,
        desc: ResourceType::Descriptor,
    ) -> VirtualResource {
        let info = ResourceInfo::new(name, handle);

        VirtualResource {
            state: ResourceState::Setup(desc.into()),
            info,
        }
    }

    pub fn imported<ResourceType: FGResource>(
        name: &str,
        handle: TypeHandle<VirtualResource>,
        resource: ImportedResource,
        desc: ResourceType::Descriptor,
    ) -> VirtualResource {
        let info = ResourceInfo::new(name, handle);

        VirtualResource {
            state: ResourceState::Imported(ImportedResourceState {
                desc: desc.into(),
                resource,
            }),
            info,
        }
    }
}

#[derive(Clone)]
pub struct ResourceInfo {
    ///唯一的资源名称
    pub name: String,
    ///资源索引
    pub handle: TypeHandle<VirtualResource>,
    /// 资源版本
    version: u32,
    ///首次使用此资源的渲染节点
    pub first_pass_node_handle: Option<TypeHandle<PassNode>>,
    ///最后使用此资源的渲染节点
    pub last_pass_node_handle: Option<TypeHandle<PassNode>>,
}

impl ResourceInfo {
    pub fn new(name: &str, handle: TypeHandle<VirtualResource>) -> Self {
        Self {
            name: name.to_string(),
            handle,
            version: 0,
            first_pass_node_handle: None,
            last_pass_node_handle: None,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn new_version(&mut self) {
        self.version += 1
    }

    pub fn update_lifetime(&mut self, handle: TypeHandle<PassNode>) {
        if self.first_pass_node_handle.is_none() {
            self.first_pass_node_handle = Some(handle);
        }

        self.last_pass_node_handle = Some(handle)
    }
}

#[derive(Clone)]
pub struct ImportedResourceState {
    pub desc: AnyFGResourceDescriptor,
    pub resource: ImportedResource,
}

#[derive(Clone)]
pub enum ImportedResource {
    Texture(Arc<Texture>),
    SwapChain(Arc<SwapChain>),
}

#[derive(Clone)]
pub enum ResourceState {
    Imported(ImportedResourceState),
    Setup(AnyFGResourceDescriptor),
}

#[derive(Debug)]
pub enum AnyFGResource {
    OwnedTexture(Texture),
    ImportedTexture(Arc<Texture>),
    ImportedSwapChain(Arc<SwapChain>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnyFGResourceDescriptor {
    Texture(TextureInfo),
    SwapChain(SwapChainInfo),
}

pub trait FGResource: 'static + Debug {
    type Descriptor: FGResourceDescriptor;

    fn borrow_resource(res: &AnyFGResource) -> &Self;
}

pub trait FGResourceDescriptor: 'static + Clone + Debug + Into<AnyFGResourceDescriptor> {
    type Resource: FGResource;
}

pub trait TypeEquals {
    type Other;
    fn same(value: Self) -> Self::Other;
}

impl<T: Sized> TypeEquals for T {
    type Other = Self;
    fn same(value: Self) -> Self::Other {
        value
    }
}

pub trait ImportToFrameGraph
where
    Self: Sized + FGResource,
{
    fn import(self: Arc<Self>) -> ImportedResource;
}
