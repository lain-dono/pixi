use pixi::{
    batch::Batch,
    image::{Image, ImageSource},
    wgpu,
    winit::{
        event::{ElementState, VirtualKeyCode, WindowEvent},
        event_loop::ControlFlow,
    },
    Target,
};

fn main() {
    pixi::run::<Basic>();
}

struct Basic {
    batch: Batch,
    bunny: Image,
}

impl pixi::Game for Basic {
    fn start(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        println!("game started");

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("init encoder"),
        });

        let bunny = ImageSource::load_srgb(&device, "examples/assets/bunny.png").unwrap();
        let bunny = Image::new("bunny.png", &mut encoder, device, &bunny);

        queue.submit(&[encoder.finish()]);

        let format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let batch = Batch::new(
            device,
            format,
            pixi::blend::NORMAL_PREMULTIPLY,
            &bunny.texture,
        );

        Self { batch, bunny }
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                match (input.virtual_keycode, input.state) {
                    (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                        *control_flow = ControlFlow::Exit
                    }
                    _ => {}
                }
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("default encoder"),
        });

        let clear_color = wgpu::Color {
            r: 0.3,
            g: 0.3,
            b: 0.4,
            a: 1.0,
        };
        pixi::clear_color(&mut encoder, &target.view, clear_color);

        let (x, y) = (10.25, 80.25);
        let (w, h) = (self.bunny.width as f32, self.bunny.height as f32);

        let min = [x, y];
        let max = [x + w, y + h];

        self.batch.add_sprite(min, max);

        self.batch.flush(&mut encoder, &device, &target);

        queue.submit(&[encoder.finish()]);
    }
}
