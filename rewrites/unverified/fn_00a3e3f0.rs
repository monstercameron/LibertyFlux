// original: 0x00A3E3F0 vehicle_accumulate_cross_product

/// Accumulate three signed two-component products from `left` and `right`
/// into the receiver's vector fields. The three-component inputs are read as
/// `f32`; destinations are `0x1060`, `0x1068`, and `0x1064` in that order.
/// For each minor, the first product is subtracted by the second in the
/// original operand order, then added to the existing destination.
///
/// The original is a thiscall method with two stack pointers. The vectors
/// and accumulators are ordinary memory, so the rewrite does not depend on
/// the source's 32-bit pointer width except at this ABI boundary.
lf_checker_rt::export!(thiscall, rw_00a3e3f0(this: u32, left: u32, right: u32) -> () {
    const ACCUMULATOR_X: u32 = 0x1060;
    const ACCUMULATOR_Y: u32 = 0x1064;
    const ACCUMULATOR_Z: u32 = 0x1068;
    const X: u32 = 0;
    const Y: u32 = 4;
    const Z: u32 = 8;

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        unsafe { f32::from_bits((address as *const u32).read_unaligned()) }
    }
    #[inline(always)]
    unsafe fn write_f32(address: u32, value: f32) {
        unsafe { (address as *mut u32).write_unaligned(value.to_bits()) }
    }
    #[inline(always)]
    fn mul(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) * core::hint::black_box(right)
    }
    #[inline(always)]
    fn sub(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }
    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }

    unsafe {
        let lx = read_f32(left.wrapping_add(X));
        let ly = read_f32(left.wrapping_add(Y));
        let lz = read_f32(left.wrapping_add(Z));
        let rx = read_f32(right.wrapping_add(X));
        let ry = read_f32(right.wrapping_add(Y));
        let rz = read_f32(right.wrapping_add(Z));

        let minor_x = sub(mul(lz, ry), mul(ly, rz));
        let minor_z = sub(mul(ly, rx), mul(lx, ry));
        let minor_y = sub(mul(lx, rz), mul(lz, rx));

        let x_field = this.wrapping_add(ACCUMULATOR_X);
        let z_field = this.wrapping_add(ACCUMULATOR_Z);
        let y_field = this.wrapping_add(ACCUMULATOR_Y);
        write_f32(x_field, add(read_f32(x_field), minor_x));
        write_f32(z_field, add(read_f32(z_field), minor_z));
        write_f32(y_field, add(read_f32(y_field), minor_y));
    }
});
