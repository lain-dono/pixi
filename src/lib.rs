pub use wgpu;

pub mod batch;
pub mod blend;
pub mod image;
pub mod layout;
pub mod math;
pub mod sprite;
pub mod target;
pub mod utils;

pub mod perf;

#[cfg(feature = "app")]
pub mod app;

pub fn clear_color(
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    clear_color: [f64; 4],
) {
    let [r, g, b, a] = clear_color;
    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment: view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color { r, g, b, a },
        }],
        depth_stencil_attachment: None,
    });
}

pub fn linear_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        compare: wgpu::CompareFunction::Undefined,
    })
}

pub fn nearest_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: 0.0,
        lod_max_clamp: 100.0,
        compare: wgpu::CompareFunction::Undefined,
    })
}

#[doc(hidden)]
#[inline(always)]
pub fn cast_slice<T>(data: &[T]) -> &[u8] {
    use std::{mem::size_of, slice::from_raw_parts};
    unsafe { from_raw_parts(data.as_ptr() as *const u8, data.len() * size_of::<T>()) }
}

fn load_module(device: &wgpu::Device, src: &[u8]) -> wgpu::ShaderModule {
    let spirv = std::io::Cursor::new(&src[..]);
    device.create_shader_module(&wgpu::read_spirv(spirv).unwrap())
}
