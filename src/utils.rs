pub(crate) fn quad_indices16() -> impl Iterator<Item = u16> {
    (0..(0x1_0000 / 4) * 6).map(|i| (i / 6 * 4 + [0, 1, 2, 0, 2, 3][i % 6]) as u16)
}

#[allow(dead_code)]
pub(crate) fn quad_indices32() -> impl Iterator<Item = u32> {
    (0..(0x1_0000_0000 / 4) * 6).map(|i| (i / 6 * 4 + [0, 1, 2, 0, 2, 3][i % 6]) as u32)
}
