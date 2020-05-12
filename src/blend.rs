use wgpu::BlendFactor::{DstAlpha, One, SrcAlpha, Zero};
use wgpu::BlendFactor::{DstColor, OneMinusSrcColor};
use wgpu::BlendFactor::{OneMinusDstAlpha, OneMinusSrcAlpha};

pub const REPLACE: Blend = Blend {
    alpha: wgpu::BlendDescriptor::REPLACE,
    color: wgpu::BlendDescriptor::REPLACE,
};

pub const PMA_NORMAL: Blend = PMA_SRC_OVER;

pub const PMA_SRC: Blend = Blend::add(One, Zero);
pub const PMA_SRC_ATOP: Blend = Blend::add(DstAlpha, OneMinusSrcAlpha);
pub const PMA_SRC_OVER: Blend = Blend::add(One, OneMinusSrcAlpha);
pub const PMA_SRC_IN: Blend = Blend::add(DstAlpha, Zero);
pub const PMA_SRC_OUT: Blend = Blend::add(OneMinusDstAlpha, Zero);

pub const PMA_DST: Blend = Blend::add(Zero, One);
pub const PMA_DST_ATOP: Blend = Blend::add(OneMinusDstAlpha, SrcAlpha);
pub const PMA_DST_OVER: Blend = Blend::add(OneMinusDstAlpha, One);
pub const PMA_DST_IN: Blend = Blend::add(Zero, SrcAlpha);
pub const PMA_DST_OUT: Blend = Blend::add(Zero, OneMinusSrcAlpha);

pub const PMA_XOR: Blend = Blend::add(OneMinusDstAlpha, OneMinusSrcAlpha);
pub const PMA_CLEAR: Blend = Blend::add(Zero, Zero);

pub const PMA_ADD: Blend = Blend::add(One, One);
pub const PMA_MUL: Blend = Blend::add_sep(DstColor, OneMinusSrcAlpha, One, OneMinusSrcAlpha);
pub const PMA_SCREEN: Blend = Blend::add_sep(One, OneMinusSrcColor, One, OneMinusSrcAlpha);

pub const NPM_NORMAL: Blend = Blend::add_sep(SrcAlpha, OneMinusSrcAlpha, One, OneMinusSrcAlpha);
pub const NPM_ADD: Blend = Blend::add_sep(SrcAlpha, One, One, One);
pub const NPM_SCREEN: Blend = Blend::add_sep(SrcAlpha, OneMinusSrcColor, One, OneMinusSrcAlpha);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Blend {
    pub alpha: wgpu::BlendDescriptor,
    pub color: wgpu::BlendDescriptor,
}

impl Blend {
    const fn add(src: wgpu::BlendFactor, dst: wgpu::BlendFactor) -> Self {
        Self::add_sep(src, dst, src, dst)
    }

    const fn add_sep(
        color_src: wgpu::BlendFactor,
        color_dst: wgpu::BlendFactor,
        alpha_src: wgpu::BlendFactor,
        alpha_dst: wgpu::BlendFactor,
    ) -> Self {
        Self {
            alpha: wgpu::BlendDescriptor {
                src_factor: alpha_src,
                dst_factor: alpha_dst,
                operation: wgpu::BlendOperation::Add,
            },
            color: wgpu::BlendDescriptor {
                src_factor: color_src,
                dst_factor: color_dst,
                operation: wgpu::BlendOperation::Add,
            },
        }
    }

    pub const fn into_color_state(self, format: wgpu::TextureFormat) -> wgpu::ColorStateDescriptor {
        wgpu::ColorStateDescriptor {
            format,
            alpha_blend: self.alpha,
            color_blend: self.color,
            write_mask: wgpu::ColorWrite::ALL,
        }
    }
}
