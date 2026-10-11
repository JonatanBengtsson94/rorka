use std::sync::Arc;

use pollster::block_on;
use thiserror::Error;
use wgpu::{
    Backends, CompositeAlphaMode, CreateSurfaceError, Device, DeviceDescriptor,
    ExperimentalFeatures, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue,
    RequestAdapterError, RequestAdapterOptions, RequestDeviceError, Surface, SurfaceConfiguration,
    TextureUsages, Trace,
};
use winit::{event_loop::OwnedDisplayHandle, window::Window};

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("failed to create surface")]
    CreateSurface(#[from] CreateSurfaceError),
    #[error("failed to request an adapter")]
    RequestAdapter(#[from] RequestAdapterError),
    #[error("failed to request a device")]
    RequestDevice(#[from] RequestDeviceError),
}

pub struct Renderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
}

impl Renderer {
    pub fn new(window: Window, display: OwnedDisplayHandle) -> Result<Renderer, RenderError> {
        let window = Arc::new(window);
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: Some(Box::new(display)),
        });

        let surface = instance.create_surface(window.clone())?;

        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: false,
        }))?;

        let (device, queue) = block_on(adapter.request_device(&DeviceDescriptor {
            label: Some("Main Device"),
            required_features: Features::empty(),
            required_limits: Limits::defaults(),
            memory_hints: Default::default(),
            experimental_features: ExperimentalFeatures::disabled(),
            trace: Trace::Off,
        }))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .or_else(|| {
                log::warn!(
                    "No sRGB surface format available, falling back to first supported format"
                );
                surface_caps.formats.first()
            })
            .copied()
            .expect("compatible adapter must support atleast one surface format");

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            color_space: wgpu::SurfaceColorSpace::Auto,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 1,
        };

        surface.configure(&device, &surface_config);

        Ok(Renderer {
            window,
            surface,
            device,
            queue,
            surface_config,
        })
    }
}
