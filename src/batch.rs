use crate::{
    image::ImageBindGroup,
    layout::{Layout, Shader, Vertex},
    target::Target,
    utils::quad_indices16,
};

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
    const MAX_QUADS: u32 = 0x1_0000 / 4;
    const MAX_INDEX: u32 = Self::MAX_QUADS * 6;

    fn new(device: &wgpu::Device) -> Self {
        let idx: Vec<u16> = quad_indices16().collect();
        Self {
            cmd_first: DrawQuad { end: 0, base: 0 },
            cmd: Vec::new(),
            vtx: Vec::new(),
            idx: device.create_buffer_with_data(crate::cast_slice(&idx), wgpu::BufferUsage::INDEX),
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.cmd_first = DrawQuad { end: 0, base: 0 };
        self.cmd.clear();
        self.vtx.clear();
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
        device.create_buffer_with_data(crate::cast_slice(&self.vtx), wgpu::BufferUsage::VERTEX)
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
    bind_group: ImageBindGroup,
}

impl Batch {
    pub fn new(
        device: &wgpu::Device,
        layout: &Layout,
        format: wgpu::TextureFormat,
        blend: crate::blend::Blend,
        bind_group: ImageBindGroup,
    ) -> Self {
        let color_state = wgpu::ColorStateDescriptor {
            format,
            alpha_blend: blend.alpha,
            color_blend: blend.color,
            write_mask: wgpu::ColorWrite::ALL,
        };

        let shader = Shader::new(device);
        let pipeline = layout.create_pipeline(device, &shader, color_state);

        Self {
            quad: QuadBatch::new(device),
            pipeline,
            bind_group,
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
        layout: &Layout,
        target: &Target,
    ) {
        if self.quad.vtx.is_empty() {
            self.clear();
            return;
        }

        let vtx = self.quad.vertex_buffer(device);

        let proj_bind_group = target.projection(device, layout);

        {
            let mut rpass = target.rpass(encoder);

            rpass.set_pipeline(&self.pipeline);
            rpass.set_vertex_buffer(0, &vtx, 0, 0);
            rpass.set_index_buffer(&self.quad.idx, 0, 0);
            rpass.set_bind_group(0, &proj_bind_group, &[]);
            rpass.set_bind_group(1, &self.bind_group, &[]);

            for cmd in self.quad.commands() {
                rpass.draw_indexed(0..cmd.end, cmd.base, 0..1);
            }
        }

        self.clear();
    }
}
