use std::sync::Arc;

use bevy::{
    ecs::resource::Resource,
    render::view::{ExtractedWindow, ExtractedWindows},
};

use crate::{FrameGraph, SwapChain, SwapChainInfo, WgpuSwapChain};

pub trait ImportedPassResource: Resource {
    fn imported(&self, frame_graph: &mut FrameGraph);
}

impl ImportedPassResource for ExtractedWindows {
    fn imported(&self, frame_graph: &mut FrameGraph) {
        for window in self.values() {
            if let Some(texture_view) = window.swap_chain_texture_view.as_deref() {
                let swap_chain = Arc::new(SwapChain::new(WgpuSwapChain {
                    texture_view: texture_view.clone(),
                }));

                let (name, info) = get_swap_chain(window);

                frame_graph.import(&name, swap_chain, info);
            }
        }
    }
}

pub fn get_swap_chain(window: &ExtractedWindow) -> (String, SwapChainInfo) {
    let name = format!("swap_chain-{}", window.entity);
    let info = SwapChainInfo { name: name.clone() };
    (name, info)
}
