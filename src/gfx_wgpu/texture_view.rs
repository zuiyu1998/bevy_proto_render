use crate::TextureViewTrait;

#[derive(Debug)]
pub struct WgpuTextureView(pub wgpu::TextureView);

impl TextureViewTrait for WgpuTextureView {}
