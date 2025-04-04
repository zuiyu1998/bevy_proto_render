use bevy::{prelude::*, render::view::ExtractedWindows};
use bevy_proto_render::{
    ColorAttachment, Pass, PassData, PassPlugin, ProtoRenderPlugin, RendererError, SwapChain,
    get_swap_chain,
};

#[derive(Debug, Resource)]
pub struct TestPass {
    pub insert_point: usize,
    pub name: String,
}

impl Default for TestPass {
    fn default() -> Self {
        TestPass {
            insert_point: 1,
            name: "test_pass".to_owned(),
        }
    }
}

#[derive(Default)]
pub struct TestPassData {
    pub main_swain_chain: String,
}

impl PassData for TestPassData {
    fn setup(&mut self, builder: &mut bevy_proto_render::PassNodeBuilder) {
        if self.main_swain_chain.is_empty() {
            return;
        }
        if let Some(swap_chain_node_handle) =
            builder.read_from_board::<SwapChain>(&self.main_swain_chain)
        {
            let swap_chain_node_ref = builder.read(swap_chain_node_handle);
            builder.add_attachment(ColorAttachment::SwapChain(swap_chain_node_ref));

            println!("setup TestPassData {}", self.main_swain_chain);
        }
    }

    fn execute(
        &self,
        _render_context: &mut bevy_proto_render::RenderContext,
    ) -> std::result::Result<(), RendererError> {
        println!("execute");

        Ok(())
    }
}

impl Pass for TestPass {
    type Data = TestPassData;

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_insert_point(&self) -> usize {
        self.insert_point
    }

    fn do_init(data: &mut Self::Data, world: &World) {
        let windows = world.resource::<ExtractedWindows>();

        let primary = windows.primary.unwrap();

        if let Some(window) = windows.windows.get(&primary) {
            let (name, _) = get_swap_chain(window);
            data.main_swain_chain = name;
        }
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        ProtoRenderPlugin,
        PassPlugin::<TestPass>::default(),
    ));

    app.add_systems(Startup, setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
