use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

use crate::render::Renderer;

use super::App;

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            log::debug!("Called resumed when there already is a renderer");
            return;
        }
        match event_loop.create_window(
            WindowAttributes::default()
                .with_title(&self.config.title)
                .with_inner_size(LogicalSize::new(self.config.width, self.config.height)),
        ) {
            Ok(window) => {
                log::info!("Window created");
                match Renderer::new(window, event_loop.owned_display_handle()) {
                    Ok(renderer) => {
                        log::info!("Renderer created");
                        self.renderer = Some(renderer);
                    }
                    Err(err) => {
                        log::error!("Failed to create renderer: {err}");
                        event_loop.exit();
                    }
                }
            }
            Err(err) => {
                log::error!("Failed to create window: {err}");
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::info!("Close event; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                log::debug!("Window redraw event; redrawing");
            }
            _ => {
                log::trace!("Unimplemented event fired: {event:?}");
            }
        }
    }
}
