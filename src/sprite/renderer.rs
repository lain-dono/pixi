use crate::{
    blend::{self, Blend},
    layout::{Layout, Shader},
};

pub struct SpritePipeline {
    pub pipeline: wgpu::RenderPipeline,
}

impl SpritePipeline {
    pub fn new(
        device: &wgpu::Device,
        layout: &Layout,
        shader: &Shader,
        format: wgpu::TextureFormat,
        blend: Blend,
    ) -> Self {
        let color_state = blend.into_color_state(format);
        let pipeline = layout.create_pipeline(device, &shader, color_state);
        Self { pipeline }
    }

    pub fn normal(
        device: &wgpu::Device,
        layout: &Layout,
        shader: &Shader,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self::new(device, layout, shader, format, blend::PMA_NORMAL)
    }

    pub fn replace(
        device: &wgpu::Device,
        layout: &Layout,
        shader: &Shader,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self::new(device, layout, shader, format, blend::REPLACE)
    }
}
