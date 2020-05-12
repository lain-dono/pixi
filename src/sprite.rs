use crate::math::{Frame, Matrix, Point};

pub mod animation;
pub mod renderer;
pub mod spritesheet;

pub use self::{animation::Animation, renderer::SpritePipeline};

//const DEFAULT_UV: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
//const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

pub struct Texture {
    //bind_group: ImageBindGroup,
    trim: Option<Frame>,
    width: f32,
    height: f32,
}

pub struct Sprite {
    transform: Matrix,
    anchor: Point,
    texture: Texture,
    round_to: Option<f32>,
}

impl Sprite {
    pub fn new(texture: Texture) -> Self {
        Self {
            transform: Matrix::IDENTITY,
            anchor: Point { x: 0.5, y: 0.5 },
            texture,
            round_to: None,
        }
    }

    #[must_use]
    pub fn with_anchor(self, anchor: Point) -> Self {
        Self { anchor, ..self }
    }

    #[must_use]
    pub fn with_round(self, scale: f32) -> Self {
        let round_to = Some(scale);
        Self { round_to, ..self }
    }

    #[must_use]
    pub fn with_transform(self, transform: Matrix) -> Self {
        Self { transform, ..self }
    }

    pub fn set_anchor(&mut self, anchor: Point) {
        self.anchor = anchor;
    }

    pub fn set_round(&mut self, round_to: Option<f32>) {
        self.round_to = round_to;
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn vertices(&self) -> [[f32; 2]; 4] {
        let wt = &self.transform;
        let (tw, th) = (self.texture.width, self.texture.height);

        let mut vertices = if let Some(trim) = self.texture.trim {
            untrimmed_vertices(wt, self.anchor, tw, th, trim)
        } else {
            trimmed_vertices(wt, self.anchor, tw, th)
        };

        if let Some(scale) = self.round_to {
            round_vertices(&mut vertices, scale)
        }

        vertices
    }
}

pub fn round_vertices(vertices: &mut [[f32; 2]; 4], scale: f32) {
    for [x, y] in vertices {
        *x = ((*x * scale.floor()) / scale).round();
        *y = ((*y * scale.floor()) / scale).round();
    }
}

pub fn untrimmed_vertices(
    wt: &Matrix,
    anchor: Point,
    tw: f32,
    th: f32,
    trim: Frame,
) -> [[f32; 2]; 4] {
    let w1 = trim.x - (anchor.x * tw);
    let w0 = w1 + trim.w;

    let h1 = trim.y - (anchor.y * th);
    let h0 = h1 + trim.h;

    [
        wt.apply(w1, h1),
        wt.apply(w0, h1),
        wt.apply(w0, h0),
        wt.apply(w1, h0),
    ]
}

pub fn trimmed_vertices(wt: &Matrix, anchor: Point, tw: f32, th: f32) -> [[f32; 2]; 4] {
    let w1 = -anchor.x * tw;
    let h1 = -anchor.y * th;

    let w0 = w1 + tw;
    let h0 = h1 + th;

    [
        wt.apply(w1, h1),
        wt.apply(w0, h1),
        wt.apply(w0, h0),
        wt.apply(w1, h0),
    ]
}
