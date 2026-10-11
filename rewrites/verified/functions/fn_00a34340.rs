// original: 0x00A34340 rotate_vector3_by_helper_pair

/// Uses the scalar at source +0x0c to obtain two helper factors, then writes
/// a transformed three-component vector. The first two components combine
/// the target's x/y values in the source instruction order; the third adds
/// source z and target z. This is a cdecl function with three pointer
/// arguments and returns the output pointer. The original reuses its second
/// incoming stack word as scratch for the first factor.
lf_checker_rt::export!(cdecl, rw_00a34340(output: u32, source: u32, target: u32) -> u32 {
    const SOURCE_X: u32 = 0;
    const SOURCE_Y: u32 = 4;
    const SOURCE_Z: u32 = 8;
    const SOURCE_FACTOR_INPUT: u32 = 0x0c;
    const TARGET_X: u32 = 0;
    const TARGET_Y: u32 = 4;
    const TARGET_Z: u32 = 8;
    const OUTPUT_X: u32 = 0;
    const OUTPUT_Y: u32 = 4;
    const OUTPUT_Z: u32 = 8;

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        let bits = unsafe { (address as *const u32).read_unaligned() };
        f32::from_bits(bits)
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
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) + core::hint::black_box(right)
    }
    #[inline(always)]
    fn sub(left: f32, right: f32) -> f32 {
        core::hint::black_box(left) - core::hint::black_box(right)
    }

    unsafe {
        let factor_input = read_f32(source + SOURCE_FACTOR_INPUT);
        let first_factor_bits =
            lf_checker_rt::callee_cdecl!(1, u32, factor_input.to_bits());
        let second_factor_bits =
            lf_checker_rt::callee_cdecl!(2, u32, factor_input.to_bits());
        let first_factor = f32::from_bits(first_factor_bits);
        let second_factor = f32::from_bits(second_factor_bits);

        let target_x = read_f32(target + TARGET_X);
        let target_y = read_f32(target + TARGET_Y);
        let source_x = read_f32(source + SOURCE_X);
        let source_y = read_f32(source + SOURCE_Y);
        let x_from_x = mul(target_x, first_factor);
        let x_from_y = mul(target_y, second_factor);
        let output_x = add(sub(x_from_x, x_from_y), source_x);

        let y_from_y = mul(target_y, first_factor);
        let y_from_x = mul(target_x, second_factor);
        let output_y = add(add(y_from_y, y_from_x), source_y);

        let source_z = read_f32(source + SOURCE_Z);
        let target_z = read_f32(target + TARGET_Z);
        let output_z = add(source_z, target_z);

        write_f32(output + OUTPUT_X, output_x);
        write_f32(output + OUTPUT_Y, output_y);
        write_f32(output + OUTPUT_Z, output_z);
        output
    }
});
