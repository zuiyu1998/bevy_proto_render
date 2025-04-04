use crate::{SwapChainTrait, TextureView};

use super::WgpuTextureView;

#[derive(Debug)]
pub struct WgpuSwapChain {
    pub texture_view: wgpu::TextureView,
}

impl SwapChainTrait for WgpuSwapChain {
    fn get_texture_view(&self) -> TextureView {
        TextureView::new(WgpuTextureView(self.texture_view.clone()))
    }
}
