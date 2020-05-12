use pixi::{
    app::{ControlFlow, EventLoop, LogicalSize, PhysicalSize, WindowBuilder, WindowEvent},
    blend::{self, Blend},
    linear_sampler,
    image::{ImageBindGroup, ImageLoader},
    layout::{Layout, Shader},
    sprite::SpritePipeline,
    target::{RenderTarget, Target},
    wgpu,
};

fn main() {
    let event_loop = EventLoop::new();

    let [width, height] = [500, 300];
    let window = WindowBuilder::new()
        .with_title("Blending & Compositing Example")
        .with_inner_size(LogicalSize::new(width, height))
        .build(&event_loop)
        .unwrap();

    pixi::app::run::<Blending>(event_loop, window, Default::default());
}

pub struct Quad {
    pub idx_buffer: wgpu::Buffer,
    pub vtx_buffer: wgpu::Buffer,
}

impl Quad {
    pub fn new(device: &wgpu::Device) -> Self {
        let (min, max) = (0.0, 100.0);

        let idx: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let vtx: [[f32; 4]; 4] = [
            [max, max, 1.0, 1.0],
            [max, min, 1.0, 0.0],
            [min, min, 0.0, 0.0],
            [min, max, 0.0, 1.0],
        ];

        let idx = device.create_buffer_with_data(pixi::cast_slice(&idx), wgpu::BufferUsage::INDEX);
        let vtx = device.create_buffer_with_data(pixi::cast_slice(&vtx), wgpu::BufferUsage::VERTEX);

        Self {
            idx_buffer: idx,
            vtx_buffer: vtx,
        }
    }
}

struct BlendPipeline {
    src: SpritePipeline,
    src_atop: SpritePipeline,
    src_over: SpritePipeline,
    src_in: SpritePipeline,
    src_out: SpritePipeline,

    dst: SpritePipeline,
    dst_atop: SpritePipeline,
    dst_over: SpritePipeline,
    dst_in: SpritePipeline,
    dst_out: SpritePipeline,

    clear: SpritePipeline,
    xor: SpritePipeline,
    add: SpritePipeline,
    mul: SpritePipeline,
    screen: SpritePipeline,
}

impl BlendPipeline {
    fn new(
        device: &wgpu::Device,
        layout: &Layout,
        shader: &Shader,
        format: wgpu::TextureFormat,
    ) -> Self {
        let create = |blend: Blend| SpritePipeline::new(device, &layout, &shader, format, blend);

        Self {
            src: create(blend::PMA_SRC),
            src_atop: create(blend::PMA_SRC_ATOP),
            src_over: create(blend::PMA_SRC_OVER),
            src_in: create(blend::PMA_SRC_IN),
            src_out: create(blend::PMA_SRC_OUT),

            dst: create(blend::PMA_DST),
            dst_atop: create(blend::PMA_DST_ATOP),
            dst_over: create(blend::PMA_DST_OVER),
            dst_in: create(blend::PMA_DST_IN),
            dst_out: create(blend::PMA_DST_OUT),

            clear: create(blend::PMA_CLEAR),
            xor: create(blend::PMA_XOR),
            add: create(blend::PMA_ADD),
            mul: create(blend::PMA_MUL),
            screen: create(blend::PMA_SCREEN),
        }
    }
}

struct Blending {
    layout: Layout,
    pipeline: BlendPipeline,
    quad: Quad,

    normal: SpritePipeline,
    replace: SpritePipeline,

    target: RenderTarget,

    bg_bind_group: ImageBindGroup,
    dst_bind_group: ImageBindGroup,
    src_bind_group: ImageBindGroup,
}

impl pixi::app::Game for Blending {
    type UserEvent = ();

    fn start(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        _size: PhysicalSize<u32>,
        _scale_factor: f64,
    ) -> Self {
        let layout = Layout::new(device);

        let path = [
            "examples/assets/blending/bg.png",
            "examples/assets/blending/dst.png",
            "examples/assets/blending/src.png",
        ];

        let sampler = linear_sampler(device);

        let mut loader = ImageLoader::new(device);
        let bg = loader.srgb_premul(device, path[0]).unwrap();
        let dst = loader.srgb_premul(device, path[1]).unwrap();
        let src = loader.srgb_premul(device, path[2]).unwrap();
        queue.submit(&[loader.finish()]);

        let bg_bind_group = layout.bind_image(device, &bg, &sampler);
        let dst_bind_group = layout.bind_image(device, &dst, &sampler);
        let src_bind_group = layout.bind_image(device, &src, &sampler);

        let shader = Shader::new(device);

        let pipeline = BlendPipeline::new(device, &layout, &shader, format);
        let target = RenderTarget::new(device, &layout, &sampler, format, 100, 100);

        let normal = SpritePipeline::normal(device, &layout, &shader, format);
        let replace = SpritePipeline::replace(device, &layout, &shader, format);

        Self {
            layout,
            pipeline,
            quad: Quad::new(device),

            normal,
            replace,

            target,

            bg_bind_group,
            dst_bind_group,
            src_bind_group,
        }
    }

    fn update(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        pixi::app::exit_helper(&event, control_flow);
    }

    fn render(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, target: Target) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("default encoder"),
        });

        pixi::clear_color(&mut encoder, &target.view, [0.0; 4]);

        fn draw_quad<'a>(
            rpass: &mut wgpu::RenderPass<'a>,
            pipeline: &'a wgpu::RenderPipeline,
            bind_group: &'a wgpu::BindGroup,
        ) {
            rpass.set_pipeline(pipeline);
            rpass.set_bind_group(1, bind_group, &[]);
            rpass.draw_indexed(0..6, 0, 0..1);
        }

        let pipelines = &[
            // src
            &self.pipeline.src,
            &self.pipeline.src_atop,
            &self.pipeline.src_over,
            &self.pipeline.src_in,
            &self.pipeline.src_out,
            // dst
            &self.pipeline.dst,
            &self.pipeline.dst_atop,
            &self.pipeline.dst_over,
            &self.pipeline.dst_in,
            &self.pipeline.dst_out,
            // other
            &self.pipeline.clear,
            &self.pipeline.xor,
            &self.pipeline.add,
            &self.pipeline.mul,
            &self.pipeline.screen,
        ];

        for (i, pipeline) in pipelines.iter().enumerate() {
            let rt_bind_group = self.target.target(1.0).projection(device, &self.layout);

            let mut rpass = self.target.clear_pass(&mut encoder);

            rpass.set_vertex_buffer(0, &self.quad.vtx_buffer, 0, 0);
            rpass.set_index_buffer(&self.quad.idx_buffer, 0, 0);
            rpass.set_bind_group(0, &rt_bind_group, &[]);

            draw_quad(
                &mut rpass,
                &self.pipeline.src.pipeline,
                &self.dst_bind_group,
            );
            draw_quad(&mut rpass, &pipeline.pipeline, &self.src_bind_group);

            drop(rpass);

            let target = Target {
                width: (100.0 * target.scale) as u32,
                height: (100.0 * target.scale) as u32,
                ..target
            };

            let proj_bind_group = target.projection(device, &self.layout);

            let scale = target.scale;
            let [w, h] = [100.0 * scale, 100.0 * scale];

            let mut rpass = target.rpass(&mut encoder);

            rpass.set_vertex_buffer(0, &self.quad.vtx_buffer, 0, 0);
            rpass.set_index_buffer(&self.quad.idx_buffer, 0, 0);
            rpass.set_bind_group(0, &proj_bind_group, &[]);

            let x = w * (i % 5) as f32;
            let y = h * (i / 5) as f32;

            rpass.set_viewport(x, y, w, h, 0.0, 1.0);

            draw_quad(&mut rpass, &self.replace.pipeline, &self.bg_bind_group);
            draw_quad(&mut rpass, &self.normal.pipeline, &self.target.bind_group);

            drop(rpass);
        }

        queue.submit(&[encoder.finish()]);
    }
}
