use std::{ops::Range, sync::Arc};

use bevy::render::render_resource::{CachedRenderPipelineId, PipelineCache};

use crate::{CommandBuffer, Device};

use super::{FGResource, GpuRead, ResourceNodeRef, ResourceTable, TransientResourceCache};

pub struct RenderContext<'a> {
    pub(crate) device: &'a Arc<Device>,
    pub(crate) resource_table: ResourceTable,
    pub(crate) transient_resource_cache: &'a mut TransientResourceCache,
    pub(crate) cb: Option<CommandBuffer>,
    pub(crate) pipeline_cache: &'a PipelineCache,
    pub(crate) queue_cbs: Vec<CommandBuffer>,
}

impl<'a> RenderContext<'a> {
    pub fn device(&self) -> &Device {
        self.device
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        if let Some(cb) = self.cb.as_mut() {
            cb.draw(vertices, instances);
        }
    }

    pub fn set_render_pipeline(&mut self, id: CachedRenderPipelineId) -> Option<()> {
        if let Some(render_pipeline) = self.pipeline_cache.get_render_pipeline(id) {
            if let Some(cb) = self.cb.as_mut() {
                cb.set_render_pipeline(render_pipeline);
            }

            Some(())
        } else {
            None
        }
    }

    pub fn set_cb(&mut self, cb: CommandBuffer) {
        self.cb = Some(cb);
    }

    pub fn take_cb(&mut self) -> Option<CommandBuffer> {
        self.cb.take()
    }

    pub fn new(
        device: &'a Arc<Device>,
        transient_resource_cache: &'a mut TransientResourceCache,
        pipeline_cache: &'a PipelineCache,
    ) -> Self {
        RenderContext {
            device,
            resource_table: ResourceTable::default(),
            transient_resource_cache,
            cb: None,
            pipeline_cache,
            queue_cbs: vec![],
        }
    }

    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &ResourceNodeRef<ResourceType, GpuRead>,
    ) -> Option<&ResourceType> {
        self.resource_table.get_resource(&handle.resource_handle())
    }
}
