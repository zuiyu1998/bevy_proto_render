use std::sync::Arc;

use crate::{CommandBuffer, Device};

use super::{FGResource, GpuRead, ResourceNodeRef, ResourceTable, TransientResourceCache};

pub struct RenderContext<'a> {
    ///资源表
    pub device: &'a Arc<Device>,
    pub resource_table: ResourceTable,
    pub transient_resource_cache: &'a mut TransientResourceCache,
    pub cb: Option<CommandBuffer>,
}

impl<'a> RenderContext<'a> {
    pub fn device(&self) -> &Device {
        self.device
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
    ) -> Self {
        RenderContext {
            device,
            resource_table: ResourceTable::default(),
            transient_resource_cache,
            cb: None,
        }
    }

    pub fn get_resource<ResourceType: FGResource>(
        &self,
        handle: &ResourceNodeRef<ResourceType, GpuRead>,
    ) -> Option<&ResourceType> {
        self.resource_table.get_resource(&handle.resource_handle())
    }
}
