pub mod render;
pub mod scene;

use render::{Quad, Renderer};
use scene::Scene;
use wgpu::
    InstanceDescriptor
;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

pub struct App {
    event_loop: Option<EventLoop<()>>,
    wgpu_instance: wgpu::Instance,
}

impl App {
    pub fn new() -> Self {
        let event_loop = Some(EventLoop::new().unwrap());

        let wgpu_instance = wgpu::Instance::new(InstanceDescriptor::default());
        Self {
            wgpu_instance,
            event_loop,
        }
    }

    pub fn run(&mut self) {
        pollster::block_on(self._run());
    }

    pub fn draw(&self, renderer: &Renderer) {
        let scene = Scene {
            quads: vec![Quad {
                position: [0.2, 0.2],
                size: [0.5, 0.5],
                color: [1.0, 0.0, 0.0],
            }],
        };

        scene.draw(renderer);
    }

    pub async fn _run(&mut self) {
        let event_loop = self.event_loop.take().unwrap();

        let window = Window::new(&event_loop).unwrap();

        let mut renderer = Renderer::new(&self.wgpu_instance, &window).await;

        // event_loop.set_control_flow(ControlFlow::Poll);
        let window = &window;
        event_loop
            .run(move |event, elwt| {
                // println!("{:?}", event);

                match event {
                    Event::WindowEvent {
                        window_id: _,
                        event,
                    } => match event {
                        WindowEvent::CloseRequested => {
                            println!("The close button was pressed; stopping");
                            elwt.exit();
                        }
                        WindowEvent::Resized(size) => {
                            println!("Resized to {:?}", size);
                            renderer.update_surface_size(size.width.max(1), size.height.max(1));

                            #[cfg(target_os = "macos")]
                            window.request_redraw();
                        }
                        WindowEvent::RedrawRequested => {
                            // Redraw the application.
                            //
                            // It's preferable for applications that do not render continuously to render in
                            // this event rather than in AboutToWait, since rendering in here allows
                            // the program to gracefully handle redraws requested by the OS.
                            self.draw(&renderer);
                        }
                        _ => (),
                    },
                    Event::AboutToWait => {
                        // Application update code.

                        // Queue a RedrawRequested event.
                        //
                        // You only need to call this if you've determined that you need to redraw in
                        // applications which do not always need to. Applications that redraw continuously
                        // can render here instead.
                        // window.request_redraw();
                    }
                    Event::LoopExiting => {
                        println!("exiting event loop...");
                    }
                    _ => (),
                }
            })
            .unwrap();
    }
}
