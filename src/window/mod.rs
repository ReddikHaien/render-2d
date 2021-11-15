use std::{os::windows::thread, thread::spawn};

use winit::{dpi::PhysicalSize, event::Event, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

pub struct WindowConfiguration{
    width: usize,
    height: usize,
    title: String,
    resizable: bool,
    on_event: fn (&Event<()>) -> ()
}

pub fn initialize_threaded_window(config: WindowConfiguration){
    spawn(move ||{
        let event_loop = EventLoop::new();
        let window = 
            WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(PhysicalSize{width: config.width as u32, height: config.height as u32})
            .with_resizable(config.resizable)
            .build(&event_loop);

        event_loop.run(move |event,target,control_flow|{
            *control_flow = ControlFlow::Wait;
            
            (config.on_event)(&event);

            match event{
                Event::WindowEvent{event,window_id} => {
                    match event{
                        winit::event::WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        });
    
    });
}