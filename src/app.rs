mod config;
mod handler;

use winit::error::EventLoopError;
use winit::event_loop::EventLoop;

pub use config::Config;

use crate::render::Renderer;

struct App {
    renderer: Option<Renderer>,
    config: Config,
}

impl App {
    fn new(config: Config) -> Self {
        Self {
            renderer: None,
            config,
        }
    }
}

pub fn run(config: Config) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    // FIXME: Do not poll until we have something to do.
    //event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(config);
    log::info!("Starting event loop");
    event_loop.run_app(&mut app)?;
    log::info!("Event loop ended");
    Ok(())
}
