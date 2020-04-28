#[derive(Clone, Copy)]
struct Transform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

struct Anchor {
    x: f32,
    y: f32,
}

impl Default for Anchor {
    fn default() -> Self {
        Self { x: 0.5, y: 0.5 }
    }
}

#[derive(Default)]
struct Size {
    width: f32,
    height: f32,
}

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

pub struct Sprite {
    world_transform: Transform,
}

impl Sprite {
    pub fn calculate_vertices(&mut self, round: Option<f32>) {
        /*
        const texture = this._texture;

        if (this._transformID === this.transform._worldID && this._textureID === texture._updateID)
        {
            return;
        }

        // update texture UV here, because base texture can be changed without calling `_onTextureUpdate`
        if (this._textureID !== texture._updateID)
        {
            this.uvs = this._texture._uvs.uvsFloat32;
        }

        this._transformID = this.transform._worldID;
        this._textureID = texture._updateID;
        */

        // set the vertex data

        let wt = self.world_transform;
        let a = wt.a;
        let b = wt.b;
        let c = wt.c;
        let d = wt.d;
        let tx = wt.tx;
        let ty = wt.ty;

        /*
        let vertexData = this.vertexData;
        let trim = texture.trim;
        let orig = texture.orig;
        let anchor = this.anchor;
        */

        let anchor = Anchor::default();
        let orig = Size::default();

        let trim: Option<Rect> = None;

        let (w0, w1, h0, h1);
        if let Some(trim) = trim {
            // if the sprite is trimmed and is not a tilingsprite then we need to add the extra
            // space before transforming the sprite coords.
            w1 = trim.x - (anchor.x * orig.width);
            w0 = w1 + trim.w;

            h1 = trim.y - (anchor.y * orig.height);
            h0 = h1 + trim.h;
        } else {
            w1 = -anchor.x * orig.width;
            w0 = w1 + orig.width;

            h1 = -anchor.y * orig.height;
            h0 = h1 + orig.height;
        }

        let mut vertex_data = [
            // xy 11
            (a * w1) + (c * h1) + tx,
            (d * h1) + (b * w1) + ty,
            // xy 10
            (a * w1) + (c * h0) + tx,
            (d * h0) + (b * w1) + ty,
            // xy 00
            (a * w0) + (c * h0) + tx,
            (d * h0) + (b * w0) + ty,
            // xy 01
            (a * w0) + (c * h1) + tx,
            (d * h1) + (b * w0) + ty,
        ];

        if let Some(scale) = round {
            for v in &mut vertex_data {
                *v = ((*v * scale.floor()) / scale).round();
            }
        }
    }
}
