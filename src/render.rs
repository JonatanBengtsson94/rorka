use pollster::block_on;
use wgpu::{Backends, Instance, InstanceDescriptor, PowerPreference, RequestAdapterOptions};

pub fn log_gpu() {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    match block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
        apply_limit_buckets: false,
    })) {
        Ok(adapter) => {
            log::debug!("Adapter: {}", adapter.get_info().name);
        }
        Err(err) => {
            log::error!("Failed to get an adapter: {err}");
            return;
        }
    }
}
