use super::Matrix;

pub const IDX_LIST: [u16; 6] = [0, 1, 2, 0, 2, 3];
pub const IDX_STRIP: [u16; 4] = [0, 1, 3, 2];

pub const VTX: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];

pub fn from_matrix_bounds(m: Matrix, [x0, y0]: [f32; 2], [x1, y1]: [f32; 2]) -> [[f32; 2]; 4] {
    [
        m.apply(x0, y0),
        m.apply(x1, y0),
        m.apply(x1, y1),
        m.apply(x0, y1),
    ]
}

/*
pub fn from_matrix(
    m: TransformMatrix,
    anchor: [f32; 2],
    frame: Frame,
) -> Self {
    let w1 = -anchor.x * frame.w;
    let w0 = w1 + frame.w;

    let h1 = -anchor.y * frame.h;
    let h0 = h1 + frame.h;

    Self::from_matrix_bounds(m, [w0, h0], [w1, h1])
}

*/

/*
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct QuadUV {
    pub uv0: [f32; 2],
    pub uv1: [f32; 2],
    pub uv2: [f32; 2],
    pub uv3: [f32; 2],
}

impl Default for QuadUV {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl QuadUV {
    pub const DEFAULT: Self = Self {
        uv0: UV::new(0.0, 0.0),
        uv1: UV::new(1.0, 0.0),
        uv2: UV::new(1.0, 1.0),
        uv3: UV::new(0.0, 1.0),
    };

    pub fn new(frame: Frame, base: Frame) -> Self {
        let Frame { x, y, w, h } = frame;
        let [tw, th] = [base.w, base.h];
        let [u0, v0] = [ x      / tw,  y      / th];
        let [u1, v1] = [(x + w) / tw, (y + h) / th];
        Self {
            uv0: UV::new(u0, v0),
            uv1: UV::new(u1, v0),
            uv2: UV::new(u1, v1),
            uv3: UV::new(u0, v1),
        }
    }

    pub fn to_array(self) -> [f32; 8] {
        [
            self.uv0.u, self.uv0.v,
            self.uv1.u, self.uv1.v,
            self.uv2.u, self.uv2.v,
            self.uv3.u, self.uv3.v,
        ]
    }

    pub fn rotated(frame: Frame, base: Frame, rotate: GD8) -> Self {
        let [tw, th] = [base.w, base.h];

        // width and height div 2 base size
        let w2 = frame.w / 2.0 / tw;
        let h2 = frame.h / 2.0 / th;

        // coordinates of center
        let cx = frame.x / tw + w2;
        let cy = frame.y / th + h2;

        // rotate 90 degrees clockwise
        let r0 = rotate + GD8::NW; // NW is top-left corner
        let r1 = rotate + GD8::NE;
        let r2 = rotate + GD8::SE;
        let r3 = rotate + GD8::SW;

        Self {
            uv0: UV::new(cx + w2 * r0.ux(), cy + h2 * r0.uy()),
            uv1: UV::new(cx + w2 * r1.ux(), cy + h2 * r1.uy()),
            uv2: UV::new(cx + w2 * r2.ux(), cy + h2 * r2.uy()),
            uv3: UV::new(cx + w2 * r3.ux(), cy + h2 * r3.uy()),
        }
    }
}
*/
