use crate::scene::BoidScene;
use ready_paint::{
    gfx::{Gfx, LimitFPS},
    Render, RenderEntry,
};
use std::sync::Arc;
use winit::{application::ApplicationHandler, dpi::PhysicalSize, window::Window};
pub async fn run() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub render: Render,
    pub first_resize: bool,
    last_frame_time: Option<std::time::Instant>,
}
impl ApplicationHandler for App {
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let RenderEntry::Ready(ref mut gfx) = self.render.entry {
            let now = std::time::Instant::now();

            if let Some(last_time) = self.last_frame_time {
                let delta_time = now - last_time;

                if let LimitFPS::Limit(fps) = gfx.limit_fps {
                    let target_frame_duration =
                        std::time::Duration::from_secs_f32(1.0 / fps as f32);

                    if delta_time < target_frame_duration {
                        // 计算下一帧的时间
                        let next_frame = last_time + target_frame_duration;
                        event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                            next_frame,
                        ));
                        return;
                    }

                    // 更新时间
                    let clamped_dt = delta_time.as_secs_f32().clamp(1.0 / 240.0, 1.0 / 30.0);
                    gfx.delta_time = clamped_dt;
                    *gfx.time.lock().unwrap() += clamped_dt;

                    // 请求重绘
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }

            self.last_frame_time = Some(now);

            // 设置下一帧的唤醒时间
            if let LimitFPS::Limit(fps) = gfx.limit_fps {
                let next_frame = now + std::time::Duration::from_secs_f32(1.0 / fps as f32);
                event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(next_frame));
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
                        self.last_frame_time = Some(std::time::Instant::now());
                        if let RenderEntry::Ready(ref mut gfx)= self.render.entry   {
                            gfx.set_zero_dt();
                        }
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
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
                if let RenderEntry::Ready(ref mut gfx) = self.render.entry {
                    // 设置下一帧的唤醒时间
                    if let LimitFPS::Limit(fps) = gfx.limit_fps {
                        let next_frame = std::time::Instant::now()
                            + std::time::Duration::from_secs_f32(1.0 / fps as f32);
                        event_loop
                            .set_control_flow(winit::event_loop::ControlFlow::WaitUntil(next_frame));
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
                        .with_inner_size(PhysicalSize::new(800, 600)),
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
