use downcast_rs::Downcast;
use std::fmt::Debug;

use crate::define_gfx_frame_graph_type;

pub trait TextureTrait: 'static + Debug + Sync + Send {}
pub trait ErasedTextureTrait: 'static + Downcast + Debug + Sync + Send {}

impl<T: TextureTrait> ErasedTextureTrait for T {}

define_gfx_frame_graph_type!(Texture, TextureTrait, ErasedTextureTrait, TextureInfo);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TextureInfo;
