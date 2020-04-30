use super::{Frame, Matrix, Point};

const fn point(x: f32, y: f32) -> Point {
    Point { x, y }
}

#[derive(Clone, Copy)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Default for Bounds {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Bounds {
    pub const DEFAULT: Self = Self {
        min: point(f32::INFINITY, f32::INFINITY),
        max: point(f32::NEG_INFINITY, f32::NEG_INFINITY),
    };

    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn from_achor(anchor: Point, frame: Frame) -> Self {
        let w1 = -anchor.x * frame.w;
        let w0 = w1 + frame.w;

        let h1 = -anchor.y * frame.h;
        let h0 = h1 + frame.h;
        Self {
            min: point(w0, h0),
            max: point(w1, h1),
        }
    }

    pub fn from_frame(frame: Frame) -> Self {
        Self {
            min: frame.min().into(),
            max: frame.max().into(),
        }
    }

    pub fn to_frame(&self) -> Frame {
        if self.is_empty() {
            return Frame::EMPTY;
        }
        Frame {
            x: self.min.x,
            y: self.min.y,
            w: self.max.x - self.min.x,
            h: self.max.y - self.min.y,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y
    }

    pub fn size(&self) -> Point {
        Point {
            x: self.max.x - self.min.x,
            y: self.max.y - self.min.y,
        }
    }

    pub fn add_xy(&mut self, x: f32, y: f32) {
        self.min.x = self.min.x.min(x);
        self.min.y = self.min.y.min(y);
        self.max.x = self.max.x.max(x);
        self.max.y = self.max.y.max(y);
    }

    pub fn add_pt(&mut self, point: [f32; 2]) {
        self.add_xy(point[0], point[1]);
    }

    pub fn add_point(&mut self, point: Point) {
        self.add_xy(point.x, point.y);
    }

    pub fn add_quad(&mut self, vertices: &[f32; 8]) {
        self.add_xy(vertices[0], vertices[1]);
        self.add_xy(vertices[2], vertices[3]);
        self.add_xy(vertices[4], vertices[5]);
        self.add_xy(vertices[6], vertices[7]);
    }

    pub fn add_frame(&mut self, matrix: &Matrix, x0: f32, y0: f32, x1: f32, y1: f32) {
        self.add_pt(matrix.apply(x0, y0));
        self.add_pt(matrix.apply(x1, y0));
        self.add_pt(matrix.apply(x1, y1));
        self.add_pt(matrix.apply(x0, y1));
    }

    pub fn add_vertex_data(&mut self, vertices: &[f32], begin: usize, end: usize) {
        for xy in vertices[begin..end].chunks_exact(2) {
            self.add_pt([xy[0], xy[1]]);
        }
    }

    pub fn add_vertices(
        &mut self,
        matrix: &Matrix,
        vertices: &[f32],
        begin: usize,
        end: usize,
        pad: Point,
    ) {
        for xy in vertices[begin..end].chunks_exact(2) {
            let xy: Point = matrix.apply(xy[0], xy[1]).into();
            self.add_point(xy - pad);
            self.add_point(xy + pad);
        }
    }

    pub fn add_bounds(&mut self, bounds: Self) {
        self.add_point(bounds.min);
        self.add_point(bounds.max);
    }

    pub fn add_bounds_mask(&mut self, bounds: Self, mask: Self) {
        let min_x = bounds.min.x.max(mask.min.x);
        let min_y = bounds.min.y.max(mask.min.y);
        let max_x = bounds.max.x.min(mask.max.x);
        let max_y = bounds.max.y.min(mask.max.y);
        if min_x <= max_x && min_y <= max_y {
            self.add_pt([min_x, min_y]);
            self.add_pt([max_x, max_y]);
        }
    }

    pub fn add_bounds_matrix(&mut self, bounds: &Self, matrix: &Matrix) {
        self.add_frame(
            matrix,
            bounds.min.x,
            bounds.min.y,
            bounds.max.x,
            bounds.max.y,
        );
    }

    pub fn add_bounds_area(&mut self, bounds: Self, frame: Frame) {
        self.add_bounds_mask(bounds, Self::from_frame(frame));
    }

    pub fn pad(&mut self, padding: Point) {
        if !self.is_empty() {
            self.min.x -= padding.x;
            self.max.x += padding.x;
            self.min.y -= padding.y;
            self.max.y += padding.y;
        }
    }

    pub fn add_frame_pad(&mut self, p0: Point, p1: Point, pad: Point) {
        self.add_point(p0 - pad);
        self.add_point(p1 + pad);
    }
}
