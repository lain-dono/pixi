#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<[f32; 4]> for Frame {
    fn from([x, y, w, h]: [f32; 4]) -> Self {
        Self { x, y, w, h }
    }
}

impl Into<[f32; 4]> for Frame {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.w, self.h]
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Frame {
    pub const EMPTY: Self = Self {
        x: 0.0,
        y: 0.0,
        w: 0.0,
        h: 0.0,
    };

    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub const fn from_wh(w: f32, h: f32) -> Self {
        Self::new(0.0, 0.0, w, h)
    }

    pub fn min(&self) -> [f32; 2] {
        [self.x, self.y]
    }
    pub fn max(&self) -> [f32; 2] {
        [self.x + self.w, self.y + self.h]
    }

    pub fn size(&self) -> [f32; 2] {
        [self.w, self.h]
    }

    pub fn pad(self, pad: f32) -> Self {
        Self {
            x: self.x - pad / 2.0,
            y: self.y - pad / 2.0,
            w: self.w + pad,
            h: self.h + pad,
        }
    }

    pub fn fit(self, other: Self) -> Self {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);

        let x2 = (self.x + self.w).min(other.x + other.w);
        let y2 = (self.y + self.h).min(other.y + other.h);

        Self {
            x: x1,
            y: y1,
            w: (x2 - x1).max(0.0),
            h: (y2 - y1).max(0.0),
        }
    }

    pub fn ceil(self, resolution: f32) -> Self {
        self.ceil_eps(resolution, 0.001)
    }

    pub fn ceil_eps(self, resolution: f32, eps: f32) -> Self {
        let x1 = ((self.x + eps) * resolution).floor() / resolution;
        let y1 = ((self.y + eps) * resolution).floor() / resolution;

        let x2 = ((self.x + self.w - eps) * resolution).ceil() / resolution;
        let y2 = ((self.y + self.h - eps) * resolution).ceil() / resolution;

        Self {
            x: x1,
            y: y1,

            w: x2 - self.x,
            h: y2 - self.y,
        }
    }
}
