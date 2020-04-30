// Your friendly neighbour https://en.wikipedia.org/wiki/Dihedral_group
//
// This file implements the dihedral group of order 16, also called
// of degree 8. That's why its called groupD8.

//import { Matrix } from './Matrix';

// Transform matrix for operation n is:
// | ux | vx |
// | uy | vy |

const UX_F32: [f32; 16] = [
    1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0,
];
const UY_F32: [f32; 16] = [
    0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0,
];
const VX_F32: [f32; 16] = [
    0.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0,
];
const VY_F32: [f32; 16] = [
    1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0, -1.0,
];

/// [Cayley Table](https://en.wikipedia.org/wiki/Cayley_table)
/// for the composition of each rotation in the dihederal group D8.
static CAYLEY: [[u8; 16]; 16] = [
    [
        0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF,
    ],
    [
        0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x0, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x8,
    ],
    [
        0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x0, 0x1, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF, 0x8, 0x9,
    ],
    [
        0x3, 0x4, 0x5, 0x6, 0x7, 0x0, 0x1, 0x2, 0xB, 0xC, 0xD, 0xE, 0xF, 0x8, 0x9, 0xA,
    ],
    [
        0x4, 0x5, 0x6, 0x7, 0x0, 0x1, 0x2, 0x3, 0xC, 0xD, 0xE, 0xF, 0x8, 0x9, 0xA, 0xB,
    ],
    [
        0x5, 0x6, 0x7, 0x0, 0x1, 0x2, 0x3, 0x4, 0xD, 0xE, 0xF, 0x8, 0x9, 0xA, 0xB, 0xC,
    ],
    [
        0x6, 0x7, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0xE, 0xF, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD,
    ],
    [
        0x7, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0xF, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE,
    ],
    [
        0x8, 0xF, 0xE, 0xD, 0xC, 0xB, 0xA, 0x9, 0x0, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1,
    ],
    [
        0x9, 0x8, 0xF, 0xE, 0xD, 0xC, 0xB, 0xA, 0x1, 0x0, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2,
    ],
    [
        0xA, 0x9, 0x8, 0xF, 0xE, 0xD, 0xC, 0xB, 0x2, 0x1, 0x0, 0x7, 0x6, 0x5, 0x4, 0x3,
    ],
    [
        0xB, 0xA, 0x9, 0x8, 0xF, 0xE, 0xD, 0xC, 0x3, 0x2, 0x1, 0x0, 0x7, 0x6, 0x5, 0x4,
    ],
    [
        0xC, 0xB, 0xA, 0x9, 0x8, 0xF, 0xE, 0xD, 0x4, 0x3, 0x2, 0x1, 0x0, 0x7, 0x6, 0x5,
    ],
    [
        0xD, 0xC, 0xB, 0xA, 0x9, 0x8, 0xF, 0xE, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0, 0x7, 0x6,
    ],
    [
        0xE, 0xD, 0xC, 0xB, 0xA, 0x9, 0x8, 0xF, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0, 0x7,
    ],
    [
        0xF, 0xE, 0xD, 0xC, 0xB, 0xA, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0,
    ],
];

/// Implements the dihedral group D8, which is similar to
/// [group D4]{@link http://mathworld.wolfram.com/DihedralGroupD4.html};
/// D8 is the same but with diagonals, and it is used for texture
/// rotations.
//
/// The directions the U- and V- axes after rotation
/// of an angle of `a: GD8Constant` are the vectors `(uX(a), uY(a))`
/// and `(vX(a), vY(a))`. These aren't necessarily unit vectors.
//
/// **Origin:**<br>
/// This is the small part of gameofbombs.com portal system. It works.
#[derive(Clone, Copy)]
pub struct GD8(u8);

impl std::ops::Add for GD8 {
    type Output = Self;

    /// Composes the two D8 operations.
    ///
    /// Taking `^` as reflection:
    ///
    /// |       | E=0 | S=2 | W=4 | N=6 | E^=8 | S^=10 | W^=12 | N^=14 |
    /// |-------|-----|-----|-----|-----|------|-------|-------|-------|
    /// | E=0   | E   | S   | W   | N   | E^   | S^    | W^    | N^    |
    /// | S=2   | S   | W   | N   | E   | S^   | W^    | N^    | E^    |
    /// | W=4   | W   | N   | E   | S   | W^   | N^    | E^    | S^    |
    /// | N=6   | N   | E   | S   | W   | N^   | E^    | S^    | W^    |
    /// | E^=8  | E^  | N^  | W^  | S^  | E    | N     | W     | S     |
    /// | S^=10 | S^  | E^  | N^  | W^  | S    | E     | N     | W     |
    /// | W^=12 | W^  | S^  | E^  | N^  | W    | S     | E     | N     |
    /// | N^=14 | N^  | W^  | S^  | E^  | N    | W     | S     | E     |
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(CAYLEY[self.0 as usize][rhs.0 as usize])
    }
}

impl std::ops::Sub for GD8 {
    type Output = Self;

    /// Reverse of `add`.
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(CAYLEY[self.0 as usize][rhs.inv().0 as usize])
    }
}

impl GD8 {
    /// East 0°
    pub const E: Self = Self(0);

    /// Southeast 45°↻
    pub const SE: Self = Self(1);

    /// South 90°↻
    pub const S: Self = Self(2);

    /// Southwest 135°↻
    pub const SW: Self = Self(3);

    /// West 180°
    pub const W: Self = Self(4);

    /// Northwest -135°/225°↻
    pub const NW: Self = Self(5);

    /// North -90°/270°↻
    pub const N: Self = Self(6);

    /// Northeast -45°/315°↻
    pub const NE: Self = Self(7);

    /// Reflection about Y-axis.
    pub const MIRROR_VERTICAL: Self = Self(8);

    /// Reflection about the main diagonal.
    pub const MAIN_DIAGONAL: Self = Self(10);

    /// Reflection about X-axis.
    pub const MIRROR_HORIZONTAL: Self = Self(12);

    /// Reflection about reverse diagonal.
    pub const REVERSE_DIAGONAL: Self = Self(14);

    /// The X-component of the U-axis after rotating the axes.
    pub fn ux(self) -> f32 {
        UX_F32[self.0 as usize]
    }

    /// The Y-component of the U-axis after rotating the axes.
    pub fn uy(self) -> f32 {
        UY_F32[self.0 as usize]
    }

    /// The X-component of the V-axis after rotating the axes.
    pub fn vx(self) -> f32 {
        VX_F32[self.0 as usize]
    }

    /// The Y-component of the V-axis after rotating the axes.
    pub fn vy(self) -> f32 {
        VY_F32[self.0 as usize]
    }

    /// The opposite symmetry of `rotation`
    #[inline(always)]
    pub fn inv(self) -> Self {
        // true only if between 8 & 15 (reflections)
        if self.0 & 8 != 0 {
            Self(self.0 % 16)
        } else {
            Self(8 - self.0 % 8)
        }
    }

    /// Adds 180 degrees to rotation, which is a commutative operation.
    pub fn rotate180(self) -> Self {
        Self(self.0 ^ 4)
    }

    /// Checks if the rotation angle is vertical, i.e. south or north.
    ///
    /// It doesn't work for reflections.
    pub fn is_vertical(self) -> bool {
        (self.0 % 4) == 2
    }

    /// Approximates the vector into one of the eight directions provided by `groupD8`.
    pub fn by_direction(dx: f32, dy: f32) -> Self {
        if dx.abs() * 2.0 <= dy.abs() {
            if dy >= 0.0 {
                Self::S
            } else {
                Self::N
            }
        } else if dy.abs() * 2.0 <= dx.abs() {
            if dx > 0.0 {
                Self::E
            } else {
                Self::W
            }
        } else if dy > 0.0 {
            if dx > 0.0 {
                Self::SE
            } else {
                Self::SW
            }
        } else if dx > 0.0 {
            Self::NE
        } else {
            Self::NW
        }
    }

    /*
        /**
         * Helps sprite to compensate texture packer rotation.
         *
         * @memberof PIXI.groupD8
         * @param {PIXI.Matrix} matrix - sprite world matrix
         * @param {PIXI.GD8Symmetry} rotation - The rotation factor to use.
         * @param {number} tx - sprite anchoring
         * @param {number} ty - sprite anchoring
         */
        fn matrix_append_rotation_inv(matrix: Matrix, rotation: GD8Symmetry, tx = 0, ty = 0) {
            // Packer used "rotation", we use "inv(rotation)"
            const mat: Matrix = rotationMatrices[groupD8.inv(rotation)];

            mat.tx = tx;
            mat.ty = ty;
            matrix.append(mat);
        }
    */
}

/*
/// Matrices for each `GD8Symmetry` rotation.
const rotationMatrices: Matrix[] = [];
*/

// Initializes `rotationCayley` and `rotationMatrices`.
// It is called only once below.
#[cfg(test)]
#[test]
fn init() {
    const UX: [i8; 16] = [1, 1, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, -1, -1, 0, 1];
    const UY: [i8; 16] = [0, 1, 1, 1, 0, -1, -1, -1, 0, 1, 1, 1, 0, -1, -1, -1];
    const VX: [i8; 16] = [0, -1, -1, -1, 0, 1, 1, 1, 0, 1, 1, 1, 0, -1, -1, -1];
    const VY: [i8; 16] = [1, 1, 0, -1, -1, -1, 0, 1, -1, -1, 0, 1, 1, 1, 0, -1];

    let mut cayley: [Vec<usize>; 16] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    for i in 0..16 {
        let row = &mut cayley[i];

        for j in 0..16 {
            // Multiplies rotation matrices i and j.
            let ux = (UX[i] * UX[j] + VX[i] * UY[j]).signum();
            let uy = (UY[i] * UX[j] + VY[i] * UY[j]).signum();
            let vx = (UX[i] * VX[j] + VX[i] * VY[j]).signum();
            let vy = (UY[i] * VX[j] + VY[i] * VY[j]).signum();

            // Finds rotation matrix matching the product and pushes it.
            for k in 0..16 {
                if UX[k] == ux && UY[k] == uy && VX[k] == vx && VY[k] == vy {
                    row.push(k);
                    break;
                }
            }
        }
    }

    for row in &cayley {
        print!("[");
        for v in row.iter() {
            print!("0x{:X}, ", v);
        }
        println!("],");
    }

    /*
    for i in 0..16 {
        const mat = new Matrix();

        mat.set(ux[i], uy[i], vx[i], vy[i], 0, 0);
        rotationMatrices.push(mat);
    }
    */
}
