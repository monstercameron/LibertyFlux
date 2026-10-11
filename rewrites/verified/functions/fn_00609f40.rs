// original: 0x00609F40 rage::phPrismaticJoint::vf3

/// Transform three child-space coordinates through the prismatic joint basis
/// stored on `this`, then add the child translation into `out`. The child
/// pointer is at +0x18 and the basis floats are at +0x200..+0x208 in the
/// shared joint layout. The third row accumulates its first two products in
/// reverse order, as required for bit-exact float results. This is a 32-bit
/// thiscall.
lf_checker_rt::export!(thiscall, rw_00609F40(this: u32, out: u32) -> u32 {
    #[inline(always)]
    fn mul_ordered(left: f32, right: f32) -> f32 { core::hint::black_box(left) * core::hint::black_box(right) }
    #[inline(always)]
    fn add_ordered(left: f32, right: f32) -> f32 { core::hint::black_box(left) + core::hint::black_box(right) }
    unsafe {
        const CHILD: u32 = 0x18;
        const BASIS: [u32; 3] = [0x200, 0x204, 0x208];
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
            let second = ((child + ROWS[row][1]) as *const f32).read_unaligned();
            let third = ((child + ROWS[row][2]) as *const f32).read_unaligned();
            let first_product = mul_ordered(first, ((this + BASIS[if row == 2 { 0 } else { 1 }]) as *const f32).read_unaligned());
            let second_product = mul_ordered(second, ((this + BASIS[if row == 2 { 1 } else { 0 }]) as *const f32).read_unaligned());
            let initial = if row == 2 { add_ordered(second_product, first_product) } else { add_ordered(first_product, second_product) };
            let third_product = mul_ordered(third, ((this + BASIS[2]) as *const f32).read_unaligned());
            result[row] = add_ordered(initial, third_product);
            ((out + row as u32 * 4) as *mut f32).write_unaligned(result[row]);
        }
        for row in 0..3 {
            let translation = ((child + TRANSLATION[row]) as *const f32).read_unaligned();
            ((out + (row as u32) * 4) as *mut f32).write_unaligned(add_ordered(translation, result[row]));
        }
        out
    }
});
