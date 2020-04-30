#[derive(Clone, Copy)]
pub struct Matrix {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub tx: f32,
    pub ty: f32,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Matrix {
    pub const IDENTITY: Self = Self::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

    pub const fn new(a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    /*
    pub fn from_transform(tr: Transform) -> Self {
        let a =  (tr.rotation + tr.skew.y).cos() * tr.scale.x;
        let b =  (tr.rotation + tr.skew.y).sin() * tr.scale.x;
        let c = -(tr.rotation - tr.skew.x).sin() * tr.scale.y;
        let d =  (tr.rotation - tr.skew.x).cos() * tr.scale.y;

        let tx = tr.position.x - (tr.pivot.x * a + tr.pivot.y * c);
        let ty = tr.position.y - (tr.pivot.x * b + tr.pivot.y * d);

        Self::new(a, b, c, d, tx, ty)
    }
    */

    pub fn from_array(array: [f32; 6]) -> Self {
        Self {
            a: array[0],
            b: array[1],
            c: array[3],
            d: array[4],
            tx: array[2],
            ty: array[5],
        }
    }

    pub fn to_mat3(self) -> [[f32; 3]; 3] {
        [
            [self.a, self.c, self.tx],
            [self.b, self.d, self.ty],
            [0.0, 0.0, 1.0],
        ]
    }

    pub fn to_mat3_transposed(self) -> [[f32; 3]; 3] {
        [
            [self.a, self.b, 0.0],
            [self.c, self.d, 0.0],
            [self.tx, self.ty, 1.0],
        ]
    }

    pub fn apply(&self, x: f32, y: f32) -> [f32; 2] {
        [
            self.a * x + self.c * y + self.tx,
            self.b * x + self.d * y + self.ty,
        ]
    }

    pub fn apply_inv(&self, x: f32, y: f32) -> [f32; 2] {
        let id = (self.a * self.d + self.c * -self.b).recip();
        [
            self.d * id * x - self.c * id * y + (self.ty * self.c - self.tx * self.d) * id,
            self.a * id * y - self.b * id * x + (self.tx * self.b - self.ty * self.a) * id,
        ]
    }

    pub fn translate(self, x: f32, y: f32) -> Self {
        Self {
            tx: self.tx + x,
            ty: self.ty + y,
            ..self
        }
    }

    pub fn scale(self, x: f32, y: f32) -> Self {
        Self {
            a: self.a * x,
            b: self.b * y,
            c: self.c * x,
            d: self.d * y,
            tx: self.tx * x,
            ty: self.ty * y,
        }
    }

    pub fn rotate(self, angle: f32) -> Self {
        let (sn, cs) = angle.sin_cos();
        Self {
            a: self.a * cs - self.b * sn,
            b: self.a * sn + self.b * cs,
            c: self.c * cs - self.d * sn,
            d: self.c * sn + self.d * cs,
            tx: self.tx * cs - self.ty * sn,
            ty: self.tx * sn + self.ty * cs,
        }
    }

    pub fn invert(&self) -> Self {
        let n = self.a * self.d - self.b * self.c;
        Self::new(
            self.d / n,
            -self.b / n,
            -self.c / n,
            self.a / n,
            (self.c * self.ty - self.d * self.tx) / n,
            (self.b * self.tx - self.a * self.ty) / n,
        )
    }

    pub fn append(self, rhs: Self) -> Self {
        Self::concat(self, rhs)
    }

    pub fn prepend(self, lhs: Self) -> Self {
        Self::concat(lhs, self)
    }

    #[inline(always)]
    fn concat(rhs: Self, lhs: Self) -> Self {
        Self {
            a: rhs.a * lhs.a + rhs.b * lhs.c,
            b: rhs.a * lhs.b + rhs.b * lhs.d,
            c: rhs.c * lhs.a + rhs.d * lhs.c,
            d: rhs.c * lhs.b + rhs.d * lhs.d,
            tx: rhs.tx * lhs.a + rhs.ty * lhs.c + lhs.tx,
            ty: rhs.tx * lhs.b + rhs.ty * lhs.d + lhs.ty,
        }
    }
}
