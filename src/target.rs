use crate::{cast_slice, layout::Layout, math::projection};

pub struct Target<'a> {
    pub view: &'a wgpu::TextureView,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
}

impl<'a> Target<'a> {
    pub fn rpass(&self, encoder: &'a mut wgpu::CommandEncoder) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Load,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color::TRANSPARENT,
            }],
            depth_stencil_attachment: None,
        })
    }

    pub fn projection(&self, device: &wgpu::Device, layout: &Layout) -> wgpu::BindGroup {
        let (width, height) = (self.width as f32, self.height as f32);
        let usage = wgpu::BufferUsage::UNIFORM;
        let matrix = projection(0.0, 0.0, width, height, self.scale);
        let buffer = device.create_buffer_with_data(cast_slice(&matrix), usage);

        layout.bind_projection(device, &buffer)
    }
}

pub struct RenderTarget {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    pub width: u32,
    pub height: u32,
}

impl RenderTarget {
    pub fn new(
        device: &wgpu::Device,
        layout: &Layout,
        sampler: &wgpu::Sampler,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("render target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        });

        let view = texture.create_default_view();
        let bind_group = layout.bind_texture(device, &view, sampler);

        Self {
            texture,
            view,
            bind_group,
            width,
            height,
        }
    }

    pub fn clear_pass<'a>(&'a self, encoder: &'a mut wgpu::CommandEncoder) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color::TRANSPARENT,
            }],
            depth_stencil_attachment: None,
        })
    }

    pub fn target(&self, scale: f32) -> Target {
        Target {
            view: &self.view,
            width: self.width,
            height: self.height,
            scale,
        }
    }
}
