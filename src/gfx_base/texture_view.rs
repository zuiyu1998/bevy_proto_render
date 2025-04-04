use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::define_gfx_type;

pub trait TextureViewTrait: 'static + Debug {}
pub trait ErasedTextureViewTrait: 'static + Downcast + Debug {}

impl<T: TextureViewTrait> ErasedTextureViewTrait for T {}

define_gfx_type!(TextureView, TextureViewTrait, ErasedTextureViewTrait);
