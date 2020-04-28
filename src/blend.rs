/*
pub enum BlendMode {
    Normal = 0,
    Add = 1,
    Multiply = 2,
    Screen = 3,
    Overlay = 4,
    Darken = 5,
    Lighten = 6,
    ColorDodge = 7,
    ColorBurn = 8,
    HardLight = 9,
    SoftLight = 10,
    Difference = 11,
    Exclusion = 12,
    Hue = 13,
    Saturation = 14,
    Color = 15,
    Luminosity = 16,
    NormalNpm = 17,
    AddNpm = 18,
    ScreenNpm = 19,
    None = 20,

    SrcOver = 0,
    SrcIn = 21,
    SrcOut = 22,
    SrcAtop = 23,
    DstOver = 24,
    DstIn = 25,
    DstOut = 26,
    DstAtop = 27,
    Erase = 26,
    Subtract = 28,
    Xor = 29,
}

pub enum CompositingMode {
    Src,
    SrcOver,
    SrcIn,

    Dst,
    DstOver,
    DstIn,

    Clear,
    SrcOut,
    SrcIn,
}
*/

pub const NORMAL_PREMULTIPLY: BlendState = BlendState::new(
    wgpu::BlendFactor::One,
    wgpu::BlendFactor::OneMinusSrcAlpha,
    wgpu::BlendOperation::Add,
);

pub const NORMAL_UNPREMULTIPLY: BlendState = BlendState::new(
    wgpu::BlendFactor::SrcAlpha,
    wgpu::BlendFactor::OneMinusSrcAlpha,
    wgpu::BlendOperation::Add,
);

pub struct BlendState {
    pub alpha_blend: wgpu::BlendDescriptor,
    pub color_blend: wgpu::BlendDescriptor,
}

impl BlendState {
    const fn new(
        src_factor: wgpu::BlendFactor,
        dst_factor: wgpu::BlendFactor,
        operation: wgpu::BlendOperation,
    ) -> Self {
        Self {
            alpha_blend: wgpu::BlendDescriptor {
                src_factor,
                dst_factor,
                operation,
            },
            color_blend: wgpu::BlendDescriptor {
                src_factor,
                dst_factor,
                operation,
            },
        }
    }
}
