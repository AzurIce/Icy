use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::Window};

pub struct App {
    event_loop: Option<EventLoop<()>>,
}

impl App {
    pub fn new() -> Self {
        let event_loop = Some(EventLoop::new().unwrap());
        Self { event_loop }
    }

    pub fn run(&mut self) {
        let event_loop = self.event_loop.take().unwrap();

        let window = Window::new(&event_loop).unwrap();

        // event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run(move |event, elwt| {
            println!("{:?}", event);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                },
                Event::AboutToWait => {
                    // Application update code.
        
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw in
                    // applications which do not always need to. Applications that redraw continuously
                    // can render here instead.
                    // window.request_redraw();
                },
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in AboutToWait, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.
                },
                Event::LoopExiting => {
                    println!("exiting event loop...");
                }
                _ => ()
            }
        }).unwrap();
    }
}