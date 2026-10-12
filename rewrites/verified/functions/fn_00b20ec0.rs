// original: 0x00B20EC0 interior_offset_distance_compute
/// Compute the midpoint between the XYZ vectors at offsets 0x00 and 0x20 of
/// `this`, plus half their Euclidean distance. Store midpoint XYZ, the
/// original's zero-filled uninitialized local word at output+0x0c, and the
/// distance at output+0x10. Subtract, multiply, and add operations follow the
/// original single-precision order; the function is thiscall with one output
/// pointer argument and no declared return value.
lf_checker_rt::export!(thiscall, rw_00b20ec0(this: u32, output: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        #[inline(always)]
        fn sub(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }
        #[inline(always)]
        fn mul(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }
        #[inline(always)]
        fn add(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) + core::hint::black_box(right)
        }
        #[inline(always)]
        unsafe fn float_at(address: u32) -> f32 {
            unsafe { f32::from_bits(read_u32(address)) }
        }

        const SECOND_VECTOR: u32 = 0x20;
        const OUTPUT_MIDPOINT_X: u32 = 0x00;
        const OUTPUT_MIDPOINT_Y: u32 = 0x04;
        const OUTPUT_MIDPOINT_Z: u32 = 0x08;
        const OUTPUT_SPILL: u32 = 0x0c;
        const OUTPUT_HALF_DISTANCE: u32 = 0x10;
        const HALF: f32 = 0.5;

        let first_x = float_at(this);
        let first_y = float_at(this.wrapping_add(4));
        let first_z = float_at(this.wrapping_add(8));
        let second_x = float_at(this.wrapping_add(SECOND_VECTOR));
        let second_y = float_at(this.wrapping_add(SECOND_VECTOR + 4));
        let second_z = float_at(this.wrapping_add(SECOND_VECTOR + 8));

        let half_dx = mul(sub(first_x, second_x), HALF);
        let half_dy = mul(sub(first_y, second_y), HALF);
        let half_dz = mul(sub(first_z, second_z), HALF);
        let midpoint_y = sub(first_y, half_dy);
        let midpoint_x = sub(first_x, half_dx);
        let squared_y = mul(half_dy, half_dy);
        let squared_x = mul(half_dx, half_dx);
        let midpoint_z = sub(first_z, half_dz);
        let squared_z = mul(half_dz, half_dz);
        let planar_square = add(squared_y, squared_x);
        let half_distance = add(planar_square, squared_z).sqrt();

        write_u32(output.wrapping_add(OUTPUT_HALF_DISTANCE), half_distance.to_bits());
        write_u32(output.wrapping_add(OUTPUT_SPILL), 0);
        write_u32(output.wrapping_add(OUTPUT_MIDPOINT_X), midpoint_x.to_bits());
        write_u32(output.wrapping_add(OUTPUT_MIDPOINT_Y), midpoint_y.to_bits());
        write_u32(output.wrapping_add(OUTPUT_MIDPOINT_Z), midpoint_z.to_bits());
        0
    }
});
