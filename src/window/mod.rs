use std::{os::windows::thread, thread::spawn};

use raw_gl_context::{GlConfig, GlContext};
use winit::{dpi::PhysicalSize, event::Event, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

use crate::resource::Resources;

pub struct WindowConfiguration{
    pub width: usize,
    pub height: usize,
    pub title: String,
    pub resizable: bool,
    pub on_event: fn (&Event<()>, &mut ControlFlow) -> (),
    pub color: Option<[f32;4]>

}

pub struct WindowResource{
    event_loop: EventLoop<()>,
    window: Window,
    ctx: GlContext
}

pub fn initialize_threaded_window(resources: &mut Resources, config: WindowConfiguration){
   
    
        let event_loop = EventLoop::new();
        let window = 
            WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(PhysicalSize{width: config.width as u32, height: config.height as u32})
            .with_resizable(config.resizable)
            .build(&event_loop)
            .expect("Failed to create window");

        let ctx = raw_gl_context::GlContext::create(&window, GlConfig::default())
        .expect("Failed to create gl context");

        ctx.make_current();
        gl::load_with(|s|ctx.get_proc_address(s));
        ctx.swap_buffers();
        if let Some(c) = config.color{
            unsafe{
                gl::ClearColor(c[0],c[1],c[2],c[3]);
            }
        }

        resources.add_resource(WindowResource{
            ctx,
            event_loop,
            window
        }, "window_resource".into());
}