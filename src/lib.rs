use bevy::{
    prelude::*,
    render::{Render, RenderApp, RenderSet},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum FrameGraphSet {
    Setup,
    Compile,
}

pub struct ProtoRenderPlugin;

impl Plugin for ProtoRenderPlugin {
    fn build(&self, _app: &mut App) {}

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        render_app.configure_sets(
            Render,
            (FrameGraphSet::Setup, FrameGraphSet::Compile)
                .chain()
                .before(RenderSet::Render),
        );
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
