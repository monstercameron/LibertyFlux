// original: 0x0088A640 stream_slot_score_from_offsets (proposed)

/// Score a slot from two packed offsets with a multiply-high division.
///
/// Reads the table base at `this + 0xe8`, forms `t = (b - 0xc0) +
/// (a * 0xffff90c0 - base)` in wrapping 32-bit signed arithmetic, then
/// returns `t / 7` computed as the original does: signed 64-bit multiply by
/// the magic constant 0x92492493, high word plus `t`, arithmetic shift right
/// 6, then add the sign bit. All arithmetic wraps exactly as the original's.
///
/// Original: 0x0088A640 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_0088a640(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const SCALE: i32 = 0xffff_90c0u32 as i32;
        const BIAS: i32 = -0xc0;
        const TABLE_BASE: u32 = 0xe8;
        const MAGIC: i64 = 0x9249_2493u32 as i32 as i64;

        let base = ((this + TABLE_BASE) as *const i32).read_unaligned();
        let t = (b as i32)
            .wrapping_add(BIAS)
            .wrapping_add((a as i32).wrapping_mul(SCALE).wrapping_sub(base));
        let prod = (t as i64).wrapping_mul(MAGIC);
        let mut q = (prod >> 32) as i32;
        q = q.wrapping_add(t);
        q >>= 6;
        q.wrapping_add((q as u32 >> 31) as i32) as u32
    }
});
