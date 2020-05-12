use crate::target::Target;
use std::time::Instant;

const CAPACITY: usize = 120;

pub struct Perf {
    ticker: Instant,
    history: [f32; CAPACITY],
    pipeline: wgpu::RenderPipeline,
}

impl Perf {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let pipeline = create_pipeline(device, format);
        Self {
            ticker: Instant::now(),
            history: [0.0; CAPACITY],
            pipeline,
        }
    }

    pub fn update(&mut self) {
        let dt = {
            let now = Instant::now();
            let dt = now.duration_since(self.ticker);
            self.ticker = now;
            dt.as_secs_f32()
        };
        self.history.rotate_left(1);
        self.history[self.history.len() - 1] = dt;
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device, target: &Target) {
        let mut lines = [[0.0; 4]; CAPACITY];

        let scale = 2.0; // target.scale
        let px = scale / target.width as f32;
        let py = scale / target.height as f32;

        let scale_x = px;
        let scale_y = py * 1000.0 * 5.0;
        for (i, &dt) in self.history.iter().enumerate() {
            let x = i as f32 * scale_x - 1.0;
            lines[i] = [x, -1.0, x, dt * scale_y - 1.0]
        }

        let data = crate::cast_slice(&lines);
        let buffer = device.create_buffer_with_data(data, wgpu::BufferUsage::VERTEX);

        let count = 2 * CAPACITY as u32;

        let mut rpass = target.rpass(encoder);
        rpass.set_vertex_buffer(0, &buffer, 0, 0);
        rpass.set_pipeline(&self.pipeline);

        rpass.draw(0..count, 0..1);
    }
}

fn create_pipeline(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    let vs = crate::load_module(device, include_bytes!("shaders/compiled/perf.vert.spv"));
    let fs = crate::load_module(device, include_bytes!("shaders/compiled/perf.frag.spv"));

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::LineList,
        color_states: &[wgpu::ColorStateDescriptor {
            format,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float2],
            }],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    })
}
