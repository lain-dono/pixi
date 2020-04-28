use super::Target;
use std::{mem::size_of, slice::from_raw_parts};

#[inline(always)]
fn cast_slice<T>(data: &[T]) -> &[u8] {
    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

fn load_shader(device: &wgpu::Device, src: &[u8]) -> wgpu::ShaderModule {
    let spirv = std::io::Cursor::new(&src[..]);
    device.create_shader_module(&wgpu::read_spirv(spirv).unwrap())
}

fn quad_indices16() -> impl Iterator<Item = u16> {
    (0..(0x1_0000 / 4) * 6).map(|i| (i / 6 * 4 + [0, 1, 2, 0, 2, 3][i % 6]) as u16)
}

fn quad_indices32() -> impl Iterator<Item = u32> {
    (0..(0x1_0000_0000 / 4) * 6).map(|i| (i / 6 * 4 + [0, 1, 2, 0, 2, 3][i % 6]) as u32)
}

fn projection(x: f32, y: f32, width: f32, height: f32, scale: f32) -> [[f32; 4]; 4] {
    let m_a = (2.0 / width) * scale;
    let m_d = (2.0 / height) * scale;
    let m_x = -1.0 - x * m_a;
    let m_y = -1.0 - y * m_d;
    [
        [m_a, 0.0, 0.0, 0.0],
        [0.0, -m_d, 0.0, 0.0],
        [m_x, -m_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

#[derive(Clone, Default)]
pub struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> Self {
        Self {
            position: [x, y],
            tex_coord: [u, v],
        }
    }

    const fn format() -> [wgpu::VertexAttributeDescriptor; 2] {
        wgpu::vertex_attr_array![0 => Float2, 1 => Float2]
    }
}

struct DrawQuad {
    end: u32,
    base: i32,
}

struct QuadBatch {
    cmd_first: DrawQuad,
    cmd: Vec<DrawQuad>,
    vtx: Vec<Vertex>,
    idx: wgpu::Buffer,
}

impl QuadBatch {
    const MAX_QUADS: u32 = 0x1_0000 / 6;
    const MAX_INDEX: u32 = Self::MAX_QUADS * 6;

    fn new(device: &wgpu::Device) -> Self {
        let idx: Vec<u16> = quad_indices16().collect();
        Self {
            cmd_first: DrawQuad { end: 0, base: 0 },
            cmd: Vec::new(),
            vtx: Vec::new(),
            idx: device.create_buffer_with_data(cast_slice(&idx), wgpu::BufferUsage::INDEX),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.cmd_first = DrawQuad { end: 0, base: 0 };
        self.cmd.clear();
    }

    #[inline]
    fn last(&self) -> &DrawQuad {
        self.cmd.last().unwrap_or(&self.cmd_first)
    }

    #[inline]
    fn last_mut(&mut self) -> &mut DrawQuad {
        self.cmd.last_mut().unwrap_or(&mut self.cmd_first)
    }

    #[inline]
    fn commands(&self) -> impl Iterator<Item = &DrawQuad> {
        std::iter::once(&self.cmd_first).chain(&self.cmd)
    }

    #[inline]
    fn vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_with_data(cast_slice(&self.vtx), wgpu::BufferUsage::VERTEX)
    }

    #[inline]
    fn add_quad(&mut self, quad: [Vertex; 4]) {
        if self.last().end >= Self::MAX_INDEX {
            let base = self.vtx.len() as i32;
            self.cmd.push(DrawQuad { end: 0, base });
        }

        let cmd = self.last_mut();
        cmd.end += 6;
        self.vtx.extend_from_slice(&quad);

        debug_assert!(self.vtx.len() <= i32::MAX as usize);
    }
}

pub struct Batch {
    quad: QuadBatch,
    pipeline: wgpu::RenderPipeline,
    projection: wgpu::BindGroupLayout,
    combined: wgpu::BindGroupLayout,
    bind: wgpu::BindGroup,
}

impl Batch {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        blend: crate::blend::BlendState,
        texture: &wgpu::Texture,
    ) -> Self {
        let color_state = wgpu::ColorStateDescriptor {
            format,
            alpha_blend: blend.alpha_blend,
            color_blend: blend.color_blend,
            write_mask: wgpu::ColorWrite::ALL,
        };

        let projection = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            bindings: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

        let combined = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Float,
                        multisampled: false,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Undefined,
        });

        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &combined,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.create_default_view()),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&projection, &combined],
        });

        let pipeline = create_pipeline(device, &layout, color_state);
        Self {
            quad: QuadBatch::new(device),
            pipeline,
            projection,
            combined,
            bind,
        }
    }

    pub fn add_quad(&mut self, quad: [Vertex; 4]) {
        self.quad.add_quad(quad);
    }

    pub fn add_sprite(&mut self, [min_x, min_y]: [f32; 2], [max_x, max_y]: [f32; 2]) {
        self.quad.add_quad([
            Vertex::new(max_x, max_y, 1.0, 1.0), // 11
            Vertex::new(max_x, min_y, 1.0, 0.0), // 10
            Vertex::new(min_x, min_y, 0.0, 0.0), // 00
            Vertex::new(min_x, max_y, 0.0, 1.0), // 01
        ])
    }

    pub fn clear(&mut self) {
        self.quad.clear();
    }

    pub fn flush(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        target: &Target,
    ) {
        if self.quad.vtx.len() == 0 {
            self.clear();
            return;
        }

        let vtx = self.quad.vertex_buffer(device);

        let proj = {
            let (width, height, scale) = (target.width as f32, target.height as f32, target.scale);
            let usage = wgpu::BufferUsage::UNIFORM;
            let matrix = projection(0.0, 0.0, width, height, scale);
            let buffer = device.create_buffer_with_data(cast_slice(&matrix), usage);

            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.projection,
                bindings: &[wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buffer,
                        range: 0..4 * 16,
                    },
                }],
            })
        };

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &target.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::TRANSPARENT,
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, &vtx, 0, 0);
            rpass.set_index_buffer(&self.quad.idx, 0, 0);
            rpass.set_bind_group(0, &proj, &[]);
            rpass.set_bind_group(1, &self.bind, &[]);

            for cmd in self.quad.commands() {
                rpass.draw_indexed(0..cmd.end, cmd.base, 0..1);
            }
        }

        self.clear();
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    color_state: wgpu::ColorStateDescriptor,
) -> wgpu::RenderPipeline {
    let vs = load_shader(device, include_bytes!("shaders/compiled/sprite.vert.spv"));
    let fs = load_shader(device, include_bytes!("shaders/compiled/sprite.frag.spv"));

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout,
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
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[color_state],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &Vertex::format(),
            }],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    })
}
