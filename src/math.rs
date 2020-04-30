mod bounds;
mod frame;
mod group_d8;
mod matrix;
mod point;
pub mod quad;

pub use self::{bounds::Bounds, frame::Frame, group_d8::GD8, matrix::Matrix, point::Point};

pub fn projection(x: f32, y: f32, width: f32, height: f32, scale: f32) -> [[f32; 4]; 4] {
    let m_a = (2.0 / width) * scale;
    let m_d = (2.0 / height) * scale;
    let m_x = -1.0 - x * m_a;
    let m_y = -1.0 - y * m_d;
    [
        [m_a, 0.0, 0.0, 0.0],
        [0.0, -m_d, 0.0, 0.0],
        [m_x, -m_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn pack_uv(u: f32, v: f32) -> [u16; 2] {
    [(clamp01(u) * 65535.0) as u16, (clamp01(v) * 65535.0) as u16]
}

#[inline(always)]
pub(crate) fn clamp01(x: f32) -> f32 {
    x.max(0.0).min(1.0)
}

pub fn create_uv(frame: Frame, [tw, th]: [f32; 2], rotate: Option<GD8>) -> [[f32; 2]; 4] {
    if let Some(rotate) = rotate {
        create_rotated_uv(frame, [tw, th], rotate)
    } else {
        create_simple_uv(frame, [tw, th])
    }
}

pub fn create_simple_uv(frame: Frame, [tw, th]: [f32; 2]) -> [[f32; 2]; 4] {
    let x0 = frame.x / tw;
    let y0 = frame.y / th;

    let x1 = (frame.x + frame.w) / tw;
    let y1 = frame.y / th;

    let x2 = (frame.x + frame.w) / tw;
    let y2 = (frame.y + frame.h) / th;

    let x3 = frame.x / tw;
    let y3 = (frame.y + frame.h) / th;

    [[x0, y0], [x1, y1], [x2, y2], [x3, y3]]
}

pub fn create_rotated_uv(frame: Frame, [tw, th]: [f32; 2], rotate: GD8) -> [[f32; 2]; 4] {
    // width and height div 2 div baseFrame size
    let w2 = frame.w / 2.0 / tw;
    let h2 = frame.h / 2.0 / th;

    // coordinates of center
    let cx = (frame.x / tw) + w2;
    let cy = (frame.y / th) + h2;

    let rotate = rotate + GD8::NW; // NW is top-left corner
    let x0 = cx + (w2 * rotate.ux());
    let y0 = cy + (h2 * rotate.uy());

    let rotate = rotate + GD8::S; // rotate 90 degrees clockwise
    let x1 = cx + (w2 * rotate.ux());
    let y1 = cy + (h2 * rotate.uy());

    let rotate = rotate + GD8::S; // rotate 90 degrees clockwise
    let x2 = cx + (w2 * rotate.ux());
    let y2 = cy + (h2 * rotate.uy());

    let rotate = rotate + GD8::S; // rotate 90 degrees clockwise
    let x3 = cx + (w2 * rotate.ux());
    let y3 = cy + (h2 * rotate.uy());

    [[x0, y0], [x1, y1], [x2, y2], [x3, y3]]
}
