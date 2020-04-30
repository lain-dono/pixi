use pixi::{
    app::winit::{dpi::PhysicalSize, event::WindowEvent, event_loop::ControlFlow},
    batch::Batch,
    blend,
    image::{Image, ImageLoader},
    sprite::Animation,
    target::Target,
    wgpu,
};
use std::time::Instant;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Animations Example");

    pixi::app::run::<Animations>(event_loop, window, Default::default());
}

struct Animations {
    images: Vec<Image>,
    batches: Vec<Batch>,
    anim: Animation<()>,
    ticker: Instant,
}

impl pixi::app::Game for Animations {
    type UserEvent = ();

    fn start(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        _size: PhysicalSize<u32>,
        _scale_factor: f64,
    ) -> Self {
        let images = [
            "examples/assets/rabbit/rabbit.png",
            "examples/assets/rabbit/rabbit_ash.png",
            "examples/assets/rabbit/rabbit_batman.png",
            "examples/assets/rabbit/rabbit_bb8.png",
            "examples/assets/rabbit/rabbit_neo.png",
            "examples/assets/rabbit/rabbit_sonic.png",
            "examples/assets/rabbit/rabbit_spidey.png",
            "examples/assets/rabbit/rabbit_stormtrooper.png",
            "examples/assets/rabbit/rabbit_superman.png",
            "examples/assets/rabbit/rabbit_tron.png",
            "examples/assets/rabbit/rabbit_wolverine.png",
            "examples/assets/rabbit/rabbit_frankenstein.png",
        ];

        let mut loader = ImageLoader::new(device);
        let images: Vec<Image> = images
            .iter()
            .map(|path| loader.srgb_premul(device, path).unwrap())
            .collect();
        queue.submit(&[loader.finish()]);

        let batches: Vec<Batch> = images
            .iter()
            .map(|image| Batch::new(device, format, blend::PMA_NORMAL, &image.texture))
            .collect();

        let mut anim = Animation::new(vec![(); batches.len()], Vec::new());
        anim.play();
        anim.set_speed(5.0);

        Self {
            images,
            batches,
            anim,
            ticker: Instant::now(),
        }
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        pixi::app::exit_helper(&event, control_flow);
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target) {
        let dt = {
            let now = Instant::now();
            let dt = now.duration_since(self.ticker);
            self.ticker = now;
            dt.as_secs_f32()
        };

        let _ = self.anim.update(dt);
        let frame = self.anim.current_frame();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("default encoder"),
        });

        let target = Target {
            scale: 8.0,
            ..target
        };

        pixi::clear_color(&mut encoder, &target.view, [0.3, 0.3, 0.4, 1.0]);

        let batch = &mut self.batches[frame];
        let w = self.images[frame].width as f32;
        let h = self.images[frame].height as f32;

        let (x, y) = (10.25, 10.25);

        let min = [x, y];
        let max = [x + w, y + h];

        batch.add_sprite(min, max);
        batch.flush(&mut encoder, &device, &target);

        queue.submit(&[encoder.finish()]);
    }
}
