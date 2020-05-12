use crate::image::{Image, ImageBindGroup};
use std::sync::Arc;

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

    pub const fn format() -> [wgpu::VertexAttributeDescriptor; 2] {
        wgpu::vertex_attr_array![0 => Float2, 1 => Float2]
    }
}

pub struct Shader {
    pub vs: wgpu::ShaderModule,
    pub fs: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(device: &wgpu::Device) -> Self {
        let vs = crate::load_module(device, include_bytes!("shaders/compiled/sprite.vert.spv"));
        let fs = crate::load_module(device, include_bytes!("shaders/compiled/sprite.frag.spv"));
        Self { vs, fs }
    }
}

pub struct Layout {
    pub projection: wgpu::BindGroupLayout,
    pub image: wgpu::BindGroupLayout,
    pub pipeline: wgpu::PipelineLayout,
}

impl Layout {
    pub fn new(device: &wgpu::Device) -> Self {
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

        let pipeline = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&projection, &combined],
        });

        Self {
            projection,
            image: combined,
            pipeline,
        }
    }

    pub fn create_pipeline(
        &self,
        device: &wgpu::Device,
        shader: &Shader,
        color_state: wgpu::ColorStateDescriptor,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &self.pipeline,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &shader.vs,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &shader.fs,
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
                    stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &Vertex::format(),
                }],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }

    pub fn bind_projection(&self, device: &wgpu::Device, buffer: &wgpu::Buffer) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("projection bind group"),
            layout: &self.projection,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buffer,
                    range: 0..4 * 16,
                },
            }],
        })
    }

    pub fn bind_texture(
        &self,
        device: &wgpu::Device,
        view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("combined bind group"),
            layout: &self.image,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    pub fn bind_image(
        &self,
        device: &wgpu::Device,
        image: &Image,
        sampler: &wgpu::Sampler,
    ) -> ImageBindGroup {
        let view = image.texture.create_default_view();
        let bind_group = self.bind_texture(device, &view, sampler);
        ImageBindGroup(Arc::new(bind_group))
    }
}
