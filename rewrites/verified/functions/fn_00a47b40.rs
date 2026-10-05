// original: 0x00a47b40 CVehicle::vf78
/// Normalised position of a global point inside a span, or 1.0 by default.
///
/// Writes 1.0 to `*out`, then returns 0 unless the enable flag is set and
/// the point lies within `[start, start+span]` (unsigned). On the inside
/// path writes `(point-start)/(span + 2^32*sign)` as f32 and returns 1
/// (thiscall, one stack argument: the output pointer). Only AL is compared.
export!(thiscall, rw_00a47b40(this: u32, out: u32) -> u32 {
    unsafe {
        const ONE: u32 = 0x3f800000;
        (out as *mut u32).write_unaligned(ONE);
        let flag = (relocated(0x0160014b) as *const u8).read();
        if flag == 0 {
            return 0;
        }
        let start = (this.wrapping_add(0x12a4) as *const u32).read_unaligned();
        let point = (relocated(0x011735b4) as *const u32).read_unaligned();
        let span = (relocated(0x0103ffd4) as *const u32).read_unaligned();
        if point > start.wrapping_add(span) {
            return 0;
        }
        if point < start {
            return 0;
        }
        let count = point.wrapping_sub(start) as i32;
        let adjust = (relocated(0x00fe8f50).wrapping_add((span >> 31) << 3) as *const u64)
            .read_unaligned();
        let num = core::hint::black_box(count as f32);
        let den = core::hint::black_box(
            (core::hint::black_box(span as i32 as f64)
                + core::hint::black_box(f64::from_bits(adjust))) as f32,
        );
        let ratio = core::hint::black_box(num) / core::hint::black_box(den);
        (out as *mut u32).write_unaligned(ratio.to_bits());
        1
    }
});
