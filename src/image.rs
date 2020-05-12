use image::DynamicImage::*;
pub use image::ImageResult;
use std::{path::Path, sync::Arc};

fn premul(texels: &mut [u8]) {
    fn mul(c: &mut u8, a: &u8) {
        *c = ((*c as f32 * *a as f32) / 255.0) as u8;
    }

    for c in texels.chunks_exact_mut(4) {
        if let [x, y, z, alpha] = c {
            mul(x, alpha);
            mul(y, alpha);
            mul(z, alpha);
        }
    }
}

#[derive(Clone)]
pub struct ImageBindGroup(pub(crate) Arc<wgpu::BindGroup>);

impl std::ops::Deref for ImageBindGroup {
    type Target = wgpu::BindGroup;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ImageSource {
    pub texels: Vec<u8>,
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
}

impl ImageSource {
    pub fn new(format: wgpu::TextureFormat, texels: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            texels,
            format,
            width,
            height,
        }
    }

    pub fn premul(mut self) -> Self {
        premul(&mut self.texels);
        self
    }

    pub fn linear(path: impl AsRef<Path>) -> ImageResult<Self> {
        let source = image::open(path)?;

        let (format, (width, height), texels) = match source {
            ImageRgba8(m) => (
                wgpu::TextureFormat::Rgba8Unorm,
                m.dimensions(),
                m.into_raw(),
            ),
            ImageBgra8(m) => (
                wgpu::TextureFormat::Bgra8Unorm,
                m.dimensions(),
                m.into_raw(),
            ),
            _ => unimplemented!(),
        };

        Ok(Self::new(format, texels, width, height))
    }

    pub fn srgb_premul(path: impl AsRef<Path>) -> ImageResult<Self> {
        Self::srgb(path).map(|m| m.premul())
    }

    pub fn srgb(path: impl AsRef<Path>) -> ImageResult<Self> {
        let source = image::open(path)?;

        let (format, (width, height), texels) = match source {
            ImageRgba8(m) => (
                wgpu::TextureFormat::Rgba8UnormSrgb,
                m.dimensions(),
                m.into_raw(),
            ),
            ImageBgra8(m) => (
                wgpu::TextureFormat::Bgra8UnormSrgb,
                m.dimensions(),
                m.into_raw(),
            ),
            _ => unimplemented!(),
        };

        Ok(Self::new(format, texels, width, height))
    }
}

pub struct Image {
    pub texture: wgpu::Texture,
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new<'a>(
        label: impl Into<Option<&'a str>>,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        source: &ImageSource,
    ) -> Self {
        let texels = device.create_buffer_with_data(&source.texels, wgpu::BufferUsage::COPY_SRC);

        let size = wgpu::Extent3d {
            width: source.width,
            height: source.height,
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: label.into(),
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: source.format,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let src = wgpu::BufferCopyView {
            buffer: &texels,
            offset: 0,
            bytes_per_row: size.width * 4,
            rows_per_image: 0,
        };
        let dst = wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d::ZERO,
        };
        encoder.copy_buffer_to_texture(src, dst, size);

        Self {
            texture,
            format: source.format,
            width: source.width,
            height: source.height,
        }
    }

    pub fn srgb_premul(
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        path: impl AsRef<Path>,
    ) -> ImageResult<Self> {
        let source = ImageSource::srgb_premul(path)?;
        Ok(Self::new(None, encoder, device, &source))
    }
}

pub struct ImageLoader {
    encoder: wgpu::CommandEncoder,
}

impl ImageLoader {
    pub fn new(device: &wgpu::Device) -> Self {
        let label = Some("ImageLoader");
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        Self { encoder }
    }

    pub fn srgb_premul(
        &mut self,
        device: &wgpu::Device,
        path: impl AsRef<Path>,
    ) -> ImageResult<Image> {
        Image::srgb_premul(&mut self.encoder, device, path)
    }

    pub fn finish(self) -> wgpu::CommandBuffer {
        self.encoder.finish()
    }
}
