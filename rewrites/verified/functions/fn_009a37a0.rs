// original: 0x009A37A0 audio_voice_level_update (proposed)

/// Update the voice output level from the tuned globals and the position.
///
/// The pitch word at `this`+0x98 (signed 16-bit) scales the global rate
/// into a base level. The sample position multiplies the global step; the
/// product truncates toward zero through the x87 unit (truncate control,
/// 64-bit store, low word kept) into a count. With UNSIGNED comparisons:
/// a first bound past the count yields the base level; a second bound at
/// or below the count, which always holds when the limit is zero (that
/// guard is unreachable dead code), yields silence. Otherwise the level is
/// `(C - (r as f32) / (d as f32)) * base` where `r` is the overrun and `d`
/// the limit, both widened from u32 through the exact integer-to-double
/// path, and `C` is the global ceiling. The level travels to the output
/// sink (callee 1, thiscall/1 on `this`+0x4C), whose answer is returned.
/// The unit's entry floating value is discarded, as the original's trailing
/// `fstp` does; the contract declares one x87 entry for it. Thiscall with
/// no stack words; float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_009A37A0(this: u32) -> u32 {
    unsafe {
        const PITCH: u32 = 0x98;
        const CURSOR: u32 = 0x14;
        const STATE: u32 = 0x24;
        const SINK: u32 = 0x4C;
        const STEP: u32 = 0x0115DBF4;
        const POSITION: u32 = 0x00FE8C58;
        const COUNT: u32 = 0x01038E50;
        const RATE: u32 = 0x01038E4C;
        const LIMIT: u32 = 0x01038E54;
        const CEIL: u32 = 0x00FE88E8;
        const U2F_MAGIC: u32 = 0x00FE8F50;
        const SINK_CALLEE: u32 = 1;
        const TWO63: f32 = 9223372036854775808.0;
        const INDEFINITE: u64 = 0x8000_0000_0000_0000;
        #[inline(always)]
        fn mul32(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub32(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div32(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn add64(a: f64, b: f64) -> f64 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        // Truncating f32 to i64 exactly as `fistp qword` under a truncate
        // control word: NaN and overflow store the indefinite pattern.
        // Values below -2^63 saturate in Rust to -2^63, whose bits equal
        // the indefinite pattern, so only NaN and +2^63 need the guard.
        #[inline(always)]
        fn fistp_trunc(f: f32) -> u64 {
            if f.is_nan() || f >= TWO63 {
                INDEFINITE
            } else {
                f as i64 as u64
            }
        }
        let g32 = |va: u32| (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned();
        let pitch = ((this.wrapping_add(PITCH)) as *const i16).read_unaligned() as i32;
        let cursor = ((this.wrapping_add(CURSOR)) as *const u32).read_unaligned();
        ((this.wrapping_add(STATE)) as *mut u32).write_unaligned(0);
        let step = f32::from_bits(g32(STEP));
        let pos = f32::from_bits(g32(POSITION));
        let scaled = mul32(pitch as f32, f32::from_bits(g32(RATE)));
        let count = fistp_trunc(mul32(step, pos)) as u32;
        let base = g32(COUNT);
        let level: f32;
        if cursor.wrapping_add(base) <= count {
            let lim = g32(LIMIT);
            if cursor.wrapping_add(lim).wrapping_add(base) <= count {
                level = 0.0;
            } else if lim == 0 {
                level = 0.0; // Unreachable: equals the first bound, which passed.
            } else {
                let r = count.wrapping_sub(cursor).wrapping_sub(base);
                let magic = |i: u32| {
                    f64::from_bits(
                        (lf_checker_rt::global::<u64>(U2F_MAGIC.wrapping_add(i * 8))
                         as *const u64)
                            .read_unaligned())
                };
                let rf = add64((r as i32) as f64, magic(r >> 31));
                let df = add64((lim as i32) as f64, magic(lim >> 31));
                let q = div32(rf as f32, df as f32);
                level = mul32(sub32(f32::from_bits(g32(CEIL)), q), scaled);
            }
        } else {
            level = scaled;
        }
        let ans = lf_checker_rt::callee_thiscall!(SINK_CALLEE, u32,
                                                  this.wrapping_add(SINK), level.to_bits());
        ((this.wrapping_add(STATE)) as *mut u32).write_unaligned(0);
        ans
    }
});
