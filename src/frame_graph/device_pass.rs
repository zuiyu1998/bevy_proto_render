use crate::{Device, RenderPassInfo, TypeHandle};

use super::{
    DynPass, FrameGraph, PassNode, RenderContext, ResourceTable, TransientResourceCache,
    VirtualResource,
};

pub struct DevicePass {
    logic_passes: Vec<LogicPass>,
    render_pass_info: RenderPassInfo,
}

pub struct LogicPass {
    pass: DynPass,
    resource_release_array: Vec<TypeHandle<VirtualResource>>,
    resource_request_array: Vec<VirtualResource>,
}

impl LogicPass {
    pub fn request_resources(
        &self,
        device: &Device,
        resource_table: &mut ResourceTable,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        for resource in self.resource_request_array.iter() {
            resource_table.request_resources(resource, device, transient_resource_cache);
        }
    }

    pub fn release_resources(
        &self,
        resource_table: &mut ResourceTable,
        transient_resource_cache: &mut TransientResourceCache,
    ) {
        for handle in self.resource_release_array.iter() {
            resource_table.release_resource(handle, transient_resource_cache);
        }
    }
}

impl Default for DevicePass {
    fn default() -> Self {
        Self::new()
    }
}

impl DevicePass {
    pub fn new() -> DevicePass {
        Self {
            logic_passes: vec![],
            render_pass_info: RenderPassInfo::default(),
        }
    }

    pub fn extra(&mut self, fg: &mut FrameGraph, handle: TypeHandle<PassNode>) {
        let pass_node = fg.get_pass_node(&handle);
        let resource_request_array = pass_node
            .resource_request_array
            .iter()
            .map(|handle| fg.get_resource(handle).clone())
            .collect();

        let pass_node = fg.get_pass_node_mut(&handle);

        let logic_pass = LogicPass {
            pass: pass_node.pass.take().unwrap(),
            resource_release_array: pass_node.resource_release_array.clone(),
            resource_request_array,
        };

        self.render_pass_info
            .color_attachments
            .append(&mut pass_node.color_attachments);

        self.logic_passes.push(logic_pass);
    }

    pub fn execute(&mut self, render_context: &mut RenderContext) {
        self.begin(render_context);

        for logic_pass in self.logic_passes.iter_mut() {
            if let Err(e) = logic_pass.pass.execute(render_context) {
                println!("{:?}", e);
            }

            logic_pass.release_resources(
                &mut render_context.resource_table,
                render_context.transient_resource_cache,
            );
        }

        self.end(render_context);
    }

    pub fn begin(&mut self, render_context: &mut RenderContext) {
        for logic_pass in self.logic_passes.iter() {
            logic_pass.request_resources(
                render_context.device,
                &mut render_context.resource_table,
                render_context.transient_resource_cache,
            );
        }

        let mut command_buffer = render_context.device().create_command_buffer();

        let mut render_pass = render_context
            .device()
            .create_render_pass(&self.render_pass_info);
        render_pass.do_init(render_context);
        command_buffer.begin_render_pass(render_context.device(), render_pass);

        render_context.set_cb(command_buffer);
    }

    pub fn end(&self, render_context: &mut RenderContext) {
        render_context.resource_table = ResourceTable::default();

        if let Some(mut command_buffer) = render_context.take_cb() {
            command_buffer.end_render_pass();

            render_context.queue_cbs.push(command_buffer);
        }
    }
}
