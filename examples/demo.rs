use bevy::prelude::*;
use bevy_proto_render::ProtoRenderPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, ProtoRenderPlugin));

    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
