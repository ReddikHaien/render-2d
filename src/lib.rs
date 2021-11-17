use std::path::Path;

use loader::texture::load_as_atlas;
use resource::Resources;
use window::{WindowConfiguration, initialize_threaded_window};
use winit::{event::{Event, WindowEvent}, event_loop::ControlFlow};

pub mod resource;
pub mod loader;
pub mod window;

#[derive(Default)]
pub struct RenderBuilderInfo{
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub title: Option<String>,
    pub event_reciever: Option<fn (&Event<()>,&mut ControlFlow) -> ()>,
    pub clear_color: Option<[f32;4]>,
    pub resizable: bool
}


///Builder for setting up the fundamentals for rendering
/// sets up render context and window
/// Texture loading and gl setup has to happen after calling build on this object
pub struct RenderWindowBuilder{
    info: RenderBuilderInfo,
    resources: Resources
}

impl RenderWindowBuilder{
    pub fn new() -> Self{
        Self{
            info: Default::default(),
            resources: Resources::new(),
        }
    }

    pub fn with_window_size(mut self, width: usize, height: usize) -> Self{
        self.info.width = Some(width);
        self.info.height = Some(height);
        self
    }

    pub fn with_window_title(mut self, title: String) -> Self{
        self.info.title = Some(title);
        self
    }

    pub fn with_event_reciever(mut self, event_reciever: fn (&Event<()>, &mut ControlFlow) -> ()) -> Self{
        self.info.event_reciever = Some(event_reciever);
        self
    }

    pub fn with_default_clear_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self{
        self.info.clear_color = Some([r,g,b,a]);
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self{
        self.info.resizable = resizable;
        self
    }

    pub fn build(mut self) -> Resources{
        initialize_threaded_window(&mut self.resources, WindowConfiguration{
            color: self.info.clear_color,
            height: self.info.height.unwrap_or(600),
            width: self.info.width.unwrap_or(800),
            title: self.info.title.unwrap_or_else(||"Title".into()),
            on_event: self.info.event_reciever.unwrap_or(default_event_reciever),
            resizable: self.info.resizable
        });
        self.resources
    }
}




fn default_event_reciever(e: &Event<()>, c: &mut ControlFlow){
    if let Event::WindowEvent{event,..  } = e{
        if let WindowEvent::CloseRequested = event{
            *c = ControlFlow::Exit;
        }
    }
}