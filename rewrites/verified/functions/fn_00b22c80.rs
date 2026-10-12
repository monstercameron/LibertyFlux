// original: 0x00B22C80 mlo_extents_clamp
/// Update three extent fields from positive input values, then clamp the
/// first to 39 when it reaches the 40 threshold and clamp the other two to
/// 200 and 300. COMISS unordered cases are preserved: NaN inputs do not
/// replace existing values, and NaN first extents are not clamped. Thiscall
/// with three stack float words.
lf_checker_rt::export!(thiscall, rw_00b22c80(this: u32, first_bits: u32, second_bits: u32, third_bits: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        const FIRST_OFFSET: u32 = 0x60;
        const SECOND_OFFSET: u32 = 0x64;
        const THIRD_OFFSET: u32 = 0x68;
        const FIRST_THRESHOLD: f32 = 40.0;
        const FIRST_LIMIT: u32 = 0x421C_0000;
        const SECOND_LIMIT: f32 = 200.0;
        const THIRD_LIMIT: f32 = 300.0;

        let first_input = f32::from_bits(first_bits);
        if first_input > 0.0 {
            write_u32(this.wrapping_add(FIRST_OFFSET), first_bits);
        }
        let first_value = f32::from_bits(read_u32(this.wrapping_add(FIRST_OFFSET)));
        if !(first_value < FIRST_THRESHOLD) {
            // COMISS sets CF for unordered, so JB takes the skip branch for a NaN.
            if !first_value.is_nan() {
                write_u32(this.wrapping_add(FIRST_OFFSET), FIRST_LIMIT);
            }
        }

        let second_input = f32::from_bits(second_bits);
        if second_input > 0.0 {
            write_u32(this.wrapping_add(SECOND_OFFSET), second_bits);
        }
        let third_input = f32::from_bits(third_bits);
        if third_input > 0.0 {
            write_u32(this.wrapping_add(THIRD_OFFSET), third_bits);
        }
        let second_value = f32::from_bits(read_u32(this.wrapping_add(SECOND_OFFSET)));
        if second_value > SECOND_LIMIT {
            write_u32(this.wrapping_add(SECOND_OFFSET), SECOND_LIMIT.to_bits());
        }
        let third_value = f32::from_bits(read_u32(this.wrapping_add(THIRD_OFFSET)));
        if third_value > THIRD_LIMIT {
            write_u32(this.wrapping_add(THIRD_OFFSET), THIRD_LIMIT.to_bits());
        }
        0
    }
});
