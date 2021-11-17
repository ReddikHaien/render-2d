use render_2d::window::{self, WindowConfiguration};
use winit::event::Event;

fn on_event(event: &Event<()>){
    match event {
        Event::WindowEvent { window_id, event } => {
            match event {
                winit::event::WindowEvent::Resized(x) => println!("{},{}",x.width,x.height),
                _ => (),
            }
        },
        _ => (),
    }
}
fn main(){
    window::initialize_threaded_window(WindowConfiguration{
        height: 600,
        on_event,
        resizable: true,
        title: "Lorem ipsum".into(),
        width: 800
    });

}