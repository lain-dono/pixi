use pixi::{
    app::winit::{dpi::PhysicalSize, event::WindowEvent, event_loop::ControlFlow},
    batch::Batch,
    blend,
    image::{Image, ImageLoader},
    target::Target,
    wgpu,
};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Basic Example");

    pixi::app::run::<Basic>(event_loop, window, Default::default());
}

struct Basic {
    batch: Batch,
    bunny: Image,
    x: [Batch; 3],
}

impl pixi::app::Game for Basic {
    type UserEvent = ();

    fn start(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        _size: PhysicalSize<u32>,
        _scale_factor: f64,
    ) -> Self {
        let images = [
            "examples/assets/bunny.png",
            "examples/assets/blending/x-red.png",
            "examples/assets/blending/x-green.png",
            "examples/assets/blending/x-blue.png",
        ];

        let mut loader = ImageLoader::new(device);
        let bunny = loader.srgb_premul(device, images[0]).unwrap();
        let xr = loader.srgb_premul(device, images[1]).unwrap();
        let xg = loader.srgb_premul(device, images[2]).unwrap();
        let xb = loader.srgb_premul(device, images[3]).unwrap();
        queue.submit(&[loader.finish()]);

        let batch = Batch::new(device, format, blend::PMA_NORMAL, &bunny.texture);

        let xr = Batch::new(device, format, blend::PMA_NORMAL, &xr.texture);
        let xg = Batch::new(device, format, blend::PMA_NORMAL, &xg.texture);
        let xb = Batch::new(device, format, blend::PMA_NORMAL, &xb.texture);
        let x = [xr, xg, xb];

        Self { batch, bunny, x }
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        pixi::app::exit_helper(&event, control_flow);
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("default encoder"),
        });

        let target = Target {
            scale: 8.0,
            ..target
        };

        pixi::clear_color(&mut encoder, &target.view, [0.3, 0.3, 0.4, 1.0]);

        let (x, y) = (10.25, 10.25);
        let (w, h) = (self.bunny.width as f32, self.bunny.height as f32);

        let min = [x, y];
        let max = [x + w, y + h];

        self.batch.add_sprite(min, max);
        self.batch.flush(&mut encoder, &device, &target);

        for (i, batch) in self.x.iter_mut().enumerate() {
            let (w, h) = (12.0, 12.0);
            let (x, y) = (40.25 + 1.5 * w * i as f32, 10.25);

            let min = [x, y];
            let max = [x + w, y + h];
            batch.add_sprite(min, max);
            batch.flush(&mut encoder, &device, &target);
        }

        queue.submit(&[encoder.finish()]);
    }
}
