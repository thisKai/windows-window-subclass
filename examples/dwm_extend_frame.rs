use {
    windows_window_subclass::{SetSubclass, DwmFrame},
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_transparent(true)
        // .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap()
        .with_subclass(DwmFrame::sheet());
    window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                },
                _ => (),
            }
            _ => (),
        }
    });
}
