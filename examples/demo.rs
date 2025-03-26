use bevy::prelude::*;
use bevy_proto_render::ProtoRenderPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, ProtoRenderPlugin));

    app.run();
}
