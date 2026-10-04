// original: 0x00892770 audSound_peak_level
/// Measure this sound's peak quantised level, or poll its live voice.
///
/// When a live voice is attached, asks it for the current level through
/// its interface (slot 6) and returns that. Otherwise walks the sound's
/// sample table, converting each entry's fixed-point magnitude to a float,
/// flooring it with the round-and-adjust sequence, truncating toward zero
/// to 64 bits (keeping the low word), and returning the largest value.
/// Returns -1 when there is nothing measurable.
export!(thiscall, rw_00892770(sound: u32) -> u32 {
    unsafe {
        const UNIT: f32 = f32::from_bits(0x3a83126f);
        // fistp qword with truncation, low 32 bits kept (invalid -> 0).
        fn fistp_low(x: f32) -> u32 {
            if x.is_nan() {
                return 0;
            }
            let t = (x.trunc()) as f64;
            if t < -9223372036854775808.0 || t >= 9223372036854775808.0 {
                0
            } else {
                (t as i64) as u32
            }
        }
        if *((sound + 0x98) as *const u32) != 0 {
            let outer = *((sound + 0x94) as *const u32);
            if outer == 0 {
                return 0xffffffff;
            }
            let inner = *(outer as *const u32);
            if inner == 0 {
                return 0xffffffff;
            }
            let table = *(inner as *const u32);
            let tgt = *((table + 0x18) as *const u32);
            let poll: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            return poll(inner);
        }
        if *((sound + 0x4d) as *const u8) == 0 {
            return 0xffffffff;
        }
        let grp = *((sound + 0x7c) as *const u32);
        if grp == 0 {
            return 0xffffffff;
        }
        let mut left = *((grp + 0x24) as *const u32);
        if left == 0 {
            return 0xffffffff;
        }
        let mut slot = *((grp + 0x14) as *const u32);
        let mut best = 0xffffffffu32;
        while left != 0 {
            let ent = *(slot as *const u32);
            let num = ((((ent + 0x10) as *const u32).read_unaligned()) as f64) as f32;
            let den =
                ((((ent + 0x18) as *const u16).read_unaligned()) as f32) * UNIT;
            let h = num / den;
            let bits = h.to_bits();
            let sign = f32::from_bits(bits & 0x80000000);
            let mag = f32::from_bits(0x4b000000 | (bits & 0x80000000));
            let m = if f32::from_bits(bits & 0x7fffffff) < 8388608.0 {
                mag
            } else {
                sign
            };
            let rounded = (h + m) - m;
            let corr = rounded - h;
            // CMPNLE: subtract only when strictly greater (or unordered).
            let floored = if !(corr <= sign) { rounded - 1.0 } else { rounded };
            let v = fistp_low(floored);
            if (v as i32) > (best as i32) {
                best = v;
            }
            slot = slot.wrapping_add(0x10);
            left -= 1;
        }
        best
    }
});
