pub use winit;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    monitor::MonitorHandle,
    window::Window,
};

pub struct Target<'a> {
    pub view: &'a wgpu::TextureView,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
}

pub trait Game: 'static + Sized {
    fn start(device: &wgpu::Device, queue: &wgpu::Queue) -> Self;
    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow);
    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target);
}

impl<T: Game> Application for T {
    type UserEvent = ();

    fn build_window(
        event_loop: &EventLoopWindowTarget<Self::UserEvent>,
        _primary_monitor: MonitorHandle,
    ) -> Window {
        Window::new(event_loop).unwrap()
    }

    fn init(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self::start(device, queue)
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        Game::update(self, event, control_flow);
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target) {
        Game::render(self, device, queue, target);
    }
}

pub trait Application: 'static + Sized {
    type UserEvent: 'static;

    fn build_window(
        event_loop: &EventLoopWindowTarget<Self::UserEvent>,
        _primary_monitor: MonitorHandle,
    ) -> Window {
        Window::new(event_loop).unwrap()
    }

    fn init(device: &wgpu::Device, queue: &wgpu::Queue) -> Self;
    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow);
    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target);
}

pub fn run<App: Application>() {
    let event_loop: EventLoop<App::UserEvent> = EventLoop::with_user_event();
    let primary_monitor = event_loop.primary_monitor();
    let window = App::build_window(&event_loop, primary_monitor);

    futures::executor::block_on(run_async::<App>(event_loop, window));
}

async fn run_async<App: Application>(event_loop: EventLoop<App::UserEvent>, window: Window) {
    let (size, surface) = {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(&window);
        (size, surface)
    };

    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        },
        wgpu::BackendBit::PRIMARY,
    )
    .await
    .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        })
        .await;

    let mut app = App::init(&device, &queue);

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let mut scale_factror = window.scale_factor();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } => {
                if window_id == window.id() {
                    match event {
                        WindowEvent::Resized(size) => {
                            sc_desc.width = size.width;
                            sc_desc.height = size.height;
                            scale_factror = window.scale_factor();
                            swap_chain = device.create_swap_chain(&surface, &sc_desc);
                        }
                        _ => (),
                    }

                    app.update(event, control_flow);
                }
            }
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(window_id) => {
                if window_id == window.id() {
                    let frame = swap_chain
                        .get_next_texture()
                        .expect("Timeout when acquiring next swap chain texture");

                    let target = Target {
                        view: &frame.view,
                        width: sc_desc.width,
                        height: sc_desc.height,
                        scale: scale_factror as f32,
                    };

                    app.render(&device, &queue, target);
                }
            }
            _ => {}
        }
    });
}
