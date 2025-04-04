use std::sync::Arc;

use crate::{SwapChain, SwapChainInfo};

use super::{
    AnyFGResource, AnyFGResourceDescriptor, FGResource, FGResourceDescriptor, ImportToFrameGraph,
    ImportedResource,
};

impl ImportToFrameGraph for SwapChain {
    fn import(self: Arc<Self>) -> ImportedResource {
        ImportedResource::SwapChain(self)
    }
}

impl FGResource for SwapChain {
    type Descriptor = SwapChainInfo;

    fn borrow_resource(res: &AnyFGResource) -> &Self {
        match res {
            AnyFGResource::ImportedSwapChain(res) => res,
            _ => {
                unimplemented!()
            }
        }
    }
}

impl From<SwapChainInfo> for AnyFGResourceDescriptor {
    fn from(value: SwapChainInfo) -> Self {
        AnyFGResourceDescriptor::SwapChain(value)
    }
}

impl FGResourceDescriptor for SwapChainInfo {
    type Resource = SwapChain;
}
