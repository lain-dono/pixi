use image::DynamicImage::*;
pub use image::ImageResult;
use std::path::Path;

pub struct ImageSource {
    pub texels: wgpu::Buffer,
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
}

impl ImageSource {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        texels: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            texels: device.create_buffer_with_data(&texels, wgpu::BufferUsage::COPY_SRC),
            format,
            width,
            height,
        }
    }

    pub fn load_linear(device: &wgpu::Device, path: impl AsRef<Path>) -> ImageResult<Self> {
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

        Ok(Self::new(device, format, texels, width, height))
    }

    pub fn load_srgb(device: &wgpu::Device, path: impl AsRef<Path>) -> ImageResult<Self> {
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

        Ok(Self::new(device, format, texels, width, height))
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
            buffer: &source.texels,
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

    pub fn load_srgb<'a>(
        label: impl Into<Option<&'a str>>,
        encoder: &mut wgpu::CommandEncoder,
        device: &wgpu::Device,
        path: impl AsRef<Path>,
    ) -> ImageResult<Self> {
        let source = ImageSource::load_srgb(device, path)?;
        Ok(Self::new(label, encoder, device, &source))
    }
}
