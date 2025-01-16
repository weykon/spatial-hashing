use crate::scene::BoidScene;
use ready_paint::{
    gfx::{Gfx, LimitFPS},
    Render, RenderEntry,
};
use std::sync::Arc;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, window::Window};
pub async fn run() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub render: Render,
    pub first_resize: bool,
}

impl ApplicationHandler for App {
    fn about_to_wait(&mut self, _: &winit::event_loop::ActiveEventLoop) {
        if let RenderEntry::Ready(ref mut gfx) = self.render.entry {
            let now = std::time::Instant::now();
            let delta_time = now - gfx.last_update;
            let time = *gfx.time.lock().unwrap() + delta_time.as_secs_f32();
            if let LimitFPS::Limit(fps) = gfx.limit_fps {
                let frame_duration = std::time::Duration::from_secs_f32(1.0 / fps as f32);
                if delta_time < frame_duration {
                    spin_sleep::sleep(frame_duration - delta_time);
                }
                gfx.last_update = std::time::Instant::now();
                *gfx.time.lock().unwrap() = time;
                let real_delta = gfx.last_update - now;
                gfx.delta_time = real_delta.as_secs_f32();
                self.window.as_ref().unwrap().request_redraw();
            }
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::Resized(size) => {
                println!("winit event: resized");
                let PhysicalSize { width, height } = size;
                match self.render.entry {
                    RenderEntry::Ready(ref mut gfx) => {
                        gfx.resize(width, height);
                        if self.first_resize {
                            self.first_resize = false;
                            return;
                        }
                        println!("resize ready");
                        self.render.ready();
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    _ => {
                        println!("resize not ready");
                    }
                }
            }
            winit::event::WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                if let RenderEntry::Ready(ref mut gfx) = self.render.entry {
                    if gfx.surface_config.is_some() {
                        self.render.paint();
                    }
                }
            }
            _ => (),
        }
    }
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let RenderEntry::NotReady = self.render.entry {
            let window = event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_title("nice_view".to_string())
                        .with_inner_size(PhysicalSize::new(600, 400)),
                )
                .unwrap();
            let main_owner_window = Arc::new(window);
            self.window = Some(main_owner_window.clone());
            let gfx = pollster::block_on(async move {
                println!("in async: Loading");
                let gfx = Gfx::new(main_owner_window.clone()).await;
                println!("in async: Ready");
                return gfx;
            });
            self.render.entry = RenderEntry::Ready(gfx);
            self.render.add_scene::<BoidScene>("check");
            self.first_resize = true;
        }
    }
}
