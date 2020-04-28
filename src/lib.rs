pub use wgpu;

pub mod batch;
pub mod blend;
pub mod image;
pub mod sprite;

#[cfg(feature = "app")]
mod app;

#[cfg(feature = "app")]
pub use self::app::{run, winit, Application, Game, Target};

pub fn clear_color(
    encoder: &mut wgpu::CommandEncoder,
    attachment: &wgpu::TextureView,
    clear_color: wgpu::Color,
) {
    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
            attachment,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color,
        }],
        depth_stencil_attachment: None,
    });
}
