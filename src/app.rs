mod swap_chain;

pub use winit;

use crate::app::swap_chain::SwapChain;
use crate::target::Target;
use winit::{
    dpi::PhysicalSize,
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub struct Options {
    pub power_preference: wgpu::PowerPreference,
    pub backends: wgpu::BackendBit,

    pub extensions: wgpu::Extensions,
    pub limits: wgpu::Limits,

    pub format: wgpu::TextureFormat,
    pub present_mode: wgpu::PresentMode,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            power_preference: wgpu::PowerPreference::Default,
            backends: wgpu::BackendBit::PRIMARY,

            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),

            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            present_mode: wgpu::PresentMode::Mailbox,
        }
    }
}

pub trait Game: 'static + Sized {
    type UserEvent: 'static;

    fn start(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        size: PhysicalSize<u32>,
        scale_factor: f64,
    ) -> Self;
    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow);
    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target);
}

pub fn exit_helper(event: &WindowEvent, control_flow: &mut ControlFlow) {
    use winit::event::{ElementState::Pressed, VirtualKeyCode::Escape};

    match event {
        WindowEvent::KeyboardInput { input, .. } => {
            if input.virtual_keycode == Some(Escape) && input.state == Pressed {
                *control_flow = ControlFlow::Exit
            }
        }
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        _ => (),
    }
}

pub fn run<App: Game>(event_loop: EventLoop<App::UserEvent>, window: Window, options: Options) {
    futures::executor::block_on(run_async::<App>(event_loop, window, options));
}

async fn run_async<App: Game>(
    event_loop: EventLoop<App::UserEvent>,
    window: Window,
    options: Options,
) {
    let (device, queue, mut swap_chain, mut app) = {
        let Options {
            power_preference,
            backends,

            extensions,
            limits,

            format,
            present_mode,
        } = options;

        let size = window.inner_size();
        let scale_factor = window.scale_factor();
        let surface = wgpu::Surface::create(&window);

        let options = wgpu::RequestAdapterOptions {
            power_preference,
            compatible_surface: Some(&surface),
        };

        let (device, queue) = wgpu::Adapter::request(&options, backends)
            .await
            .unwrap()
            .request_device(&wgpu::DeviceDescriptor { extensions, limits })
            .await;

        let sc = SwapChain::new(&device, surface, size, scale_factor, format, present_mode);
        let app = App::start(&device, &queue, format, size, scale_factor);
        (device, queue, sc, app)
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::NewEvents(StartCause::Init) => {}
            Event::NewEvents(StartCause::Poll) => {}
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {}
            Event::NewEvents(StartCause::WaitCancelled { .. }) => {}

            Event::WindowEvent { event, window_id } => {
                if window_id == window.id() {
                    if let WindowEvent::Resized(size) = event {
                        swap_chain.resize(&device, size)
                    }
                    if let WindowEvent::ScaleFactorChanged { scale_factor, .. } = event {
                        swap_chain.scale_factor = scale_factor;
                    }

                    app.update(event, control_flow);
                }
            }

            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_event) => {}
            Event::Suspended => {}
            Event::Resumed => {}

            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(window_id) => {
                if window_id == window.id() {
                    let frame = swap_chain.next_frame();

                    let target = Target {
                        view: &frame.view,
                        width: swap_chain.size.width,
                        height: swap_chain.size.height,
                        scale: swap_chain.scale_factor as f32,
                    };

                    app.render(&device, &queue, target);
                }
            }
            Event::RedrawEventsCleared => {}

            Event::LoopDestroyed => {}
        }
    });
}
