use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::define_gfx_type;

use super::TextureView;

pub trait SwapChainTrait: 'static + Debug + Send + Sync {
    fn get_texture_view(&self) -> TextureView;
}

pub trait ErasedSwapChainTrait: 'static + Downcast + Debug + Send + Sync {
    fn get_texture_view(&self) -> TextureView;
}

impl<T: SwapChainTrait> ErasedSwapChainTrait for T {
    fn get_texture_view(&self) -> TextureView {
        <T as SwapChainTrait>::get_texture_view(self)
    }
}

define_gfx_type!(SwapChain, SwapChainTrait, ErasedSwapChainTrait);

impl SwapChain {
    pub fn get_texture_view(&self) -> TextureView {
        self.value.get_texture_view()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SwapChainInfo {
    pub name: String,
}
