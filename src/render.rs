use std::sync::Arc;

use pollster::block_on;
use thiserror::Error;
use wgpu::{
    Backends, Device, Instance, InstanceDescriptor, PowerPreference, Queue, RequestAdapterError,
    RequestAdapterOptions, Surface, SurfaceConfiguration,
};
use winit::window::Window;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("failed to request an adapter")]
    RequestAdapter(#[from] RequestAdapterError),
}

pub struct Renderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, RenderError> {
        let window = Arc::new(window);
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
            apply_limit_buckets: false,
        }))?;
    }
}
