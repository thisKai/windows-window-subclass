use {
    windows_custom_window::{
        SetSubclass,
        ExtendFrame,
        ClientArea,
        HitTest,
        Margins,
        window_frame_metrics,
    },
    winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
};

fn main() {
    let event_loop = EventLoop::new();

    let metrics = window_frame_metrics().unwrap_or_default();
    let window = WindowBuilder::new()
        .with_visible(false)
        .with_transparent(true)
        // .with_no_redirection_bitmap(true)
        .build(&event_loop)
        .unwrap()
        .with_subclass(ExtendFrame::margins(Margins {
            top: metrics.titlebar,
            ..Default::default()
        }))
        .with_subclass(ClientArea::margins(Margins {
            top: metrics.titlebar,
            ..Default::default()
        }))
        .with_subclass(HitTest::extend_titlebar(metrics.titlebar));

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
