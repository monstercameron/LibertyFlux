// original: 0x00609C60 rage::phJoint3Dof::vf3

/// Transform three child-space coordinates through the 3-degree-of-freedom
/// joint basis stored on `this`, then add the child translation into `out`.
/// The shared `RagePhJoint` layout identifies the basis at 0x210..0x218 and
/// the child pointer at 0x1c. The original's first two products in the third
/// row are accumulated in reverse order from the first two rows; that order
/// is preserved for bit-exact IEEE results. This is a 32-bit thiscall.
lf_checker_rt::export!(thiscall, rw_00609C60(this: u32, out: u32) -> u32 {
    #[inline(always)]
    fn mul_ordered(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn add_ordered(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    unsafe {
        const CHILD: u32 = 0x1c;
        const BASIS: [u32; 3] = [0x210, 0x214, 0x218];
        const ROWS: [[u32; 3]; 3] = [
            [0x1a4, 0x1a0, 0x1a8],
            [0x1b4, 0x1b0, 0x1b8],
            [0x1c0, 0x1c4, 0x1c8],
        ];
        const TRANSLATION: [u32; 3] = [0x1d0, 0x1d4, 0x1d8];
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let mut result = [0.0f32; 3];
        for row in 0..3 {
            let first = ((child + ROWS[row][0]) as *const f32).read_unaligned();
            let first_basis = ((this + BASIS[if row == 2 { 0 } else { 1 }]) as *const f32).read_unaligned();
            let second = ((child + ROWS[row][1]) as *const f32).read_unaligned();
            let second_basis = ((this + BASIS[if row == 2 { 1 } else { 0 }]) as *const f32).read_unaligned();
            let third = ((child + ROWS[row][2]) as *const f32).read_unaligned();
            let third_basis = ((this + BASIS[2]) as *const f32).read_unaligned();
            let first_product = mul_ordered(first, first_basis);
            let second_product = mul_ordered(second, second_basis);
            let initial = if row == 2 {
                add_ordered(second_product, first_product)
            } else {
                add_ordered(first_product, second_product)
            };
            let third_product = mul_ordered(third, third_basis);
            result[row] = add_ordered(initial, third_product);
            ((out + (row as u32) * 4) as *mut f32).write_unaligned(result[row]);
        }
        for row in 0..3 {
            let translation = ((child + TRANSLATION[row]) as *const f32).read_unaligned();
            let translated = add_ordered(translation, result[row]);
            ((out + (row as u32) * 4) as *mut f32).write_unaligned(translated);
        }
        out
    }
});
