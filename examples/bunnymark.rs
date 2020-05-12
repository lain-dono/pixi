use pixi::{
    app::{ControlFlow, EventLoop, PhysicalSize, Window, WindowEvent},
    batch::Batch,
    blend,
    image::{Image, ImageLoader},
    layout::Layout,
    perf::Perf,
    target::Target,
    wgpu,
};
use std::time::Instant;

fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("BunnyMark");

    pixi::app::run::<Basic>(event_loop, window, Default::default());
}

struct Basic {
    perf: Perf,
    batch: Batch,
    rabbit: Image,
    layout: Layout,
    entities: Vec<Entity>,
    ticker: Instant,
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
        let path = [
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
        let rabbit = loader.srgb_premul(device, path[0]).unwrap();
        queue.submit(&[loader.finish()]);

        let layout = Layout::new(device);
        let sampler = pixi::linear_sampler(device);

        let bind_group = layout.bind_image(device, &rabbit, &sampler);

        let batch = Batch::new(device, &layout, format, blend::PMA_NORMAL, bind_group);

        let count = 100_000;
        let mut entities = Vec::with_capacity(count);
        for _ in 0..count {
            entities.push(Entity::new());
        }

        let perf = Perf::new(device, format);

        Self {
            perf,

            batch,
            rabbit,
            layout,
            entities,

            ticker: Instant::now(),
        }
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        pixi::app::exit_helper(&event, control_flow);
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target) {
        self.perf.update();

        let dt = {
            let now = Instant::now();
            let dt = now.duration_since(self.ticker);
            self.ticker = now;
            dt.as_secs_f32()
        };

        let bounds = Bounds {
            left: 0.0,
            top: 0.0,
            right: target.width as f32 / target.scale,
            bottom: target.height as f32 / target.scale,
        };

        for e in &mut self.entities {
            e.update(&bounds, 0.75, dt);
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("default encoder"),
        });

        pixi::clear_color(&mut encoder, &target.view, [0.3, 0.3, 0.4, 1.0]);

        let (w, h) = (self.rabbit.width as f32, self.rabbit.height as f32);
        for e in &self.entities {
            let (x, y) = (e.position.x, e.position.y);
            self.batch.add_sprite([x, y], [x + w, y + h]);
        }

        self.batch
            .flush(&mut encoder, &device, &self.layout, &target);

        self.perf.draw(&mut encoder, device, &target);

        queue.submit(&[encoder.finish()]);
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

struct Bounds {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

struct Entity {
    position: Point,
    speed: Point,
}

impl Entity {
    fn new() -> Self {
        let position = Point {
            x: rand01() * 800.0,
            y: 7.0,
        };
        let speed = Point {
            x: rand01() * 10.0,
            y: (rand01() * 10.0) - 5.0,
        };
        Self { position, speed }
    }

    fn update(&mut self, bounds: &Bounds, gravity: f32, step: f32) {
        let scale = 25.0;

        self.position.x += self.speed.x * step * scale;
        self.position.y += self.speed.y * step * scale;
        self.speed.y += gravity * step * scale;

        if self.position.x > bounds.right {
            self.speed.x *= -1.0;
            self.position.x = bounds.right;
        } else if self.position.x < bounds.left {
            self.speed.x *= -1.0;
            self.position.x = bounds.left;
        }

        if self.position.y > bounds.bottom {
            self.speed.y *= -0.85;
            self.position.y = bounds.bottom;
            if rand01() > 0.5 {
                self.speed.y -= rand01() * 6.0;
            }
        } else if self.position.y < bounds.top {
            self.speed.y = 0.0;
            self.position.y = bounds.top;
        }
    }
}

// In real code you should use `rand` crate.
fn rand01() -> f32 {
    static mut STATE: u32 = 4; // chosen by fair dice roll.
                               // guaranteed to be random.
    unsafe {
        // see https://en.wikipedia.org/wiki/Lehmer_random_number_generator
        STATE = ((STATE as u64 * 48271) % 0x7fffffff) as u32;
        (STATE as f32) / (0x7fffffff as f32)
    }
}
