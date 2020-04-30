use pixi::{
    app::winit::{dpi::PhysicalSize, event::WindowEvent, event_loop::ControlFlow},
    blend::{self, Blend},
    default_sampler,
    image::ImageLoader,
    layout::{Layout, Shader},
    math::projection,
    target::RenderTarget,
    target::Target,
    wgpu,
};

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let [width, height] = [500, 300];
    let window = winit::window::WindowBuilder::new()
        .with_title("Blending & Compositing Example")
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
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

struct Pipeline {
    src: wgpu::RenderPipeline,
    src_atop: wgpu::RenderPipeline,
    src_over: wgpu::RenderPipeline,
    src_in: wgpu::RenderPipeline,
    src_out: wgpu::RenderPipeline,

    dst: wgpu::RenderPipeline,
    dst_atop: wgpu::RenderPipeline,
    dst_over: wgpu::RenderPipeline,
    dst_in: wgpu::RenderPipeline,
    dst_out: wgpu::RenderPipeline,

    clear: wgpu::RenderPipeline,
    xor: wgpu::RenderPipeline,
    add: wgpu::RenderPipeline,
    mul: wgpu::RenderPipeline,
    screen: wgpu::RenderPipeline,
}

impl Pipeline {
    fn new(
        device: &wgpu::Device,
        layout: &Layout,
        shader: &Shader,
        format: wgpu::TextureFormat,
    ) -> Self {
        let create =
            |blend: Blend| layout.create_pipeline(device, &shader, blend.into_color_state(format));

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
    pipeline: Pipeline,
    quad: Quad,

    replace_pipeline: wgpu::RenderPipeline,
    normal_pipeline: wgpu::RenderPipeline,

    target: RenderTarget,

    bg_bind_group: wgpu::BindGroup,
    dst_bind_group: wgpu::BindGroup,
    src_bind_group: wgpu::BindGroup,
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

        let images = [
            "examples/assets/blending/bg.png",
            "examples/assets/blending/dst.png",
            "examples/assets/blending/src.png",
        ];

        let sampler = default_sampler(device);

        let mut loader = ImageLoader::new(device);
        let bg = loader.srgb_premul(device, images[0]).unwrap();
        let dst = loader.srgb_premul(device, images[1]).unwrap();
        let src = loader.srgb_premul(device, images[2]).unwrap();
        queue.submit(&[loader.finish()]);

        let bg = bg.texture.create_default_view();
        let src = src.texture.create_default_view();
        let dst = dst.texture.create_default_view();

        let bg_bind_group = layout.bind_combined(device, &bg, &sampler);
        let dst_bind_group = layout.bind_combined(device, &dst, &sampler);
        let src_bind_group = layout.bind_combined(device, &src, &sampler);

        let shader = Shader::new(device);

        let pipeline = Pipeline::new(device, &layout, &shader, format);
        let target = RenderTarget::new(device, &layout, &sampler, format, 100, 100);

        let shader = Shader::new(device);
        let normal_pipeline =
            layout.create_pipeline(device, &shader, blend::PMA_NORMAL.into_color_state(format));
        let replace_pipeline =
            layout.create_pipeline(device, &shader, blend::REPLACE.into_color_state(format));

        Self {
            layout,
            pipeline,
            quad: Quad::new(device),

            normal_pipeline,
            replace_pipeline,

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
            let target_bind_group = self.target.bind_projection(device, &self.layout, 1.0);
            let mut rpass = self.target.pass(&mut encoder);

            rpass.set_vertex_buffer(0, &self.quad.vtx_buffer, 0, 0);
            rpass.set_index_buffer(&self.quad.idx_buffer, 0, 0);
            rpass.set_bind_group(0, &target_bind_group, &[]);

            draw_quad(&mut rpass, &self.pipeline.src, &self.dst_bind_group);
            draw_quad(&mut rpass, pipeline, &self.src_bind_group);

            drop(rpass);

            let scale = target.scale;
            let [w, h] = [100.0 * scale, 100.0 * scale];

            let proj_bind_group = {
                let [width, height, scale] = [w, h, scale];
                let usage = wgpu::BufferUsage::UNIFORM;
                let data = projection(0.0, 0.0, width, height, scale);
                let buffer = device.create_buffer_with_data(pixi::cast_slice(&data), usage);
                self.layout.bind_projection(device, &buffer)
            };

            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &target.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_vertex_buffer(0, &self.quad.vtx_buffer, 0, 0);
            rpass.set_index_buffer(&self.quad.idx_buffer, 0, 0);
            rpass.set_bind_group(0, &proj_bind_group, &[]);

            let x = w * (i % 5) as f32;
            let y = h * (i / 5) as f32;

            rpass.set_viewport(x, y, w, h, 0.0, 1.0);

            draw_quad(&mut rpass, &self.replace_pipeline, &self.bg_bind_group);
            draw_quad(&mut rpass, &self.normal_pipeline, &self.target.bind_group);

            drop(rpass);
        }

        queue.submit(&[encoder.finish()]);
    }
}
