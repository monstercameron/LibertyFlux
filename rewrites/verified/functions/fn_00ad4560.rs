// original: 0x00AD4560 audio_find_or_add_pair (proposed)

/// Find an audio pair entry or append it, clamping the keys.
///
/// Clamps each of the two integer keys into [-3000, 3000] (comparing
/// through floats; a clamped key also zeroes the value slot), then scans
/// the pair table (8-byte entries: two half-word keys, one float) for an
/// entry matching both keys and an ordered-equal float. A match returns
/// its index; otherwise the triple is appended, the count incremented and
/// the old count returned. Cdecl/3 (key_x, key_y, value bits).
lf_checker_rt::export!(cdecl, rw_00ad4560(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const LO_BOUND: u32 = 0x00FE8E08;
        const HI_BOUND: u32 = 0x00FE8C78;
        const COUNT: u32 = 0x0154E304;
        const TABLE: u32 = 0x0158E860;
        const LO_CLAMP: i32 = -3000;
        const HI_CLAMP: i32 = 3000;
        const ENTRY: u32 = 8;
        let c_lo = lf_checker_rt::global::<f32>(LO_BOUND).read();
        let c_hi = lf_checker_rt::global::<f32>(HI_BOUND).read();
        // Ordered comparisons throughout: the original's jb is also taken on
        // unordered (NaN) inputs, so a NaN bound never clamps. `!(a < b)`
        // would wrongly clamp there; `a >= b` is false for NaN, as needed.
        let mut x = a0 as i32;
        let mut slot = f32::from_bits(a2);
        if c_lo >= x as f32 {
            x = LO_CLAMP;
            slot = 0.0;
        }
        if (x as f32) >= c_hi {
            x = HI_CLAMP;
            slot = 0.0;
        }
        let mut y = a1 as i32;
        if c_lo >= y as f32 {
            y = LO_CLAMP;
            slot = 0.0;
        }
        if (y as f32) >= c_hi {
            y = HI_CLAMP;
            slot = 0.0;
        }
        let count = lf_checker_rt::global::<u32>(COUNT).read();
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let base = lf_checker_rt::relocated(TABLE).wrapping_add(i.wrapping_mul(ENTRY));
            if (base as *const i16).read() as i32 != x {
                i += 1;
                continue;
            }
            if (base.wrapping_add(2) as *const i16).read() as i32 != y {
                i += 1;
                continue;
            }
            if (base.wrapping_add(4) as *const f32).read() == slot {
                return i;
            }
            i += 1;
        }
        let base = lf_checker_rt::relocated(TABLE).wrapping_add(count.wrapping_mul(ENTRY));
        (base as *mut u16).write(x as u16);
        (base.wrapping_add(2) as *mut u16).write(y as u16);
        (base.wrapping_add(4) as *mut u32).write(slot.to_bits());
        lf_checker_rt::global::<u32>(COUNT).write(count.wrapping_add(1));
        count
    }
});
