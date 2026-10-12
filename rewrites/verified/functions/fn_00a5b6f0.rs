// original: 0x00A5B6F0 average_four_corners

/// Averages four stored X/Y corner components in source order, writes the
/// second thiscall float argument to the Z component at +8, and scales both
/// sums by the global quarter factor. Each addition is left-associative as in
/// the original SSE sequence. ECX is the three-component result record; three
/// words are removed from the caller's stack. EAX is not a semantic result.
lf_checker_rt::export!(thiscall, rw_00A5B6F0(this: u32, _first: u32, z_bits: u32, _third: u32) -> u32 {
    const OUTPUT_X: u32 = 0;
    const OUTPUT_Y: u32 = 4;
    const OUTPUT_Z: u32 = 8;
    const FIRST_CORNER_X: u32 = 0x10;
    const FIRST_CORNER_Y: u32 = 0x14;
    const NEXT_CORNER_X: [u32; 2] = [0x20, 0x30];
    const NEXT_CORNER_Y: [u32; 2] = [0x24, 0x34];
    const FOURTH_CORNER: u32 = 0x40;
    const QUARTER_VALUE_VA: u32 = 0x00fe87e4;

    #[inline(always)]
    fn add_f32(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left + right
    }

    #[inline(always)]
    fn multiply_f32(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        left * right
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    let mut sum_x = unsafe { read_f32(this.wrapping_add(FIRST_CORNER_X)) };
    let mut sum_y = unsafe { read_f32(this.wrapping_add(FIRST_CORNER_Y)) };
    for offset in 0..2 {
        sum_x = add_f32(sum_x, unsafe { read_f32(this.wrapping_add(NEXT_CORNER_X[offset])) });
        sum_y = add_f32(sum_y, unsafe { read_f32(this.wrapping_add(NEXT_CORNER_Y[offset])) });
    }
    sum_x = add_f32(sum_x, unsafe { read_f32(this.wrapping_add(FOURTH_CORNER)) });
    sum_y = add_f32(sum_y, unsafe { read_f32(this.wrapping_add(FOURTH_CORNER + 4)) });

    let quarter = unsafe { lf_checker_rt::global::<f32>(QUARTER_VALUE_VA).read_unaligned() };
    let average_x = multiply_f32(sum_x, quarter);
    let average_y = multiply_f32(sum_y, quarter);
    unsafe {
        write_u32(this.wrapping_add(OUTPUT_X), average_x.to_bits());
        write_u32(this.wrapping_add(OUTPUT_Y), average_y.to_bits());
        write_u32(this.wrapping_add(OUTPUT_Z), z_bits);
    }
    0
});
