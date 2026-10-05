// original: 0x009aa3f0 table_scaled_random
/// Draw a random value scaled by a Stride25 row's weight, capped.
///
/// Looks up row `index` in the Stride25 table (`this+0x9d0`, count at
/// table `+0x0a`): a missing table, a negative or out-of-range index,
/// or a negative weight word at row start `+0x14` falls back to a
/// plain random draw between 6500 and 10000 (stubbed, cdecl/2).
/// Otherwise a unit random factor is drawn (stubbed, cdecl/2 with the
/// float bits of 0.8 and 1.2, float ST0 out) and multiplied by the
/// weight as a float (original operand order, pinned against
/// commuting). The product is truncated toward zero to 64 bits
/// exactly like the original's x87 `fistp` (NaN, infinities and
/// out-of-range magnitudes yield the indefinite 0x8000000000000000),
/// and the low 32 bits are returned, capped above at 50000 by an
/// unsigned comparison (so negative truncations cap too). Thiscall,
/// one stack word, dword result.
export!(thiscall, rw_009AA3F0(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_PTR: u32 = 0x9d0;
        const ROW_COUNT: u32 = 0x0a;
        const ROW_STRIDE: u32 = 25;
        const WEIGHT_OFF: u32 = 0x14;
        const CAP: u32 = 0xc350;
        const LO: u32 = 0x3f4ccccd;
        const HI: u32 = 0x3f99999a;
        let table = ((this + TABLE_PTR) as *const u32).read_unaligned();
        let weight = if table == 0 {
            return callee_cdecl!(2, u32, 0x1964, 0x2710);
        } else if (index as i32) < 0 {
            return callee_cdecl!(2, u32, 0x1964, 0x2710);
        } else {
            let count = ((table + ROW_COUNT) as *const u16).read_unaligned() as u32;
            if index >= count {
                return callee_cdecl!(2, u32, 0x1964, 0x2710);
            }
            let w = ((table + index * ROW_STRIDE + WEIGHT_OFF) as *const i32).read_unaligned();
            if w < 0 {
                return callee_cdecl!(2, u32, 0x1964, 0x2710);
            }
            w as f32
        };
        let r: f32 = callee_cdecl!(1, f32, LO, HI);
        let a = core::hint::black_box(r);
        let b = core::hint::black_box(weight);
        let p = a * b;
        let t = fistp_trunc_low32(p);
        if t > CAP {
            CAP
        } else {
            t
        }
    }
});

/// Truncate `p` toward zero to 64 bits like x87 `fistp`, low word.
///
/// Values that the x87 unit flags invalid (NaN, infinities, and
/// magnitudes at or beyond 2^63) yield the indefinite integer
/// 0x8000000000000000; everything else truncates, and the low 32
/// bits are returned. (Minus 2^63 truncates to 0x8000000000000000
/// too, so both arms agree there.)
fn fistp_trunc_low32(p: f32) -> u32 {
    const LIM: f32 = 9223372036854775808.0;
    const INDEF: i64 = i64::MIN;
    let v: i64 = if p.is_nan() || p >= LIM || p <= -LIM {
        INDEF
    } else {
        p as i64
    };
    v as u32
}
