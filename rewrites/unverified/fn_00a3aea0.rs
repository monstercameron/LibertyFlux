// original: 0x00a3aea0 vehicle_lane_ready (proposed)

/// Decide whether a lane record is ready through layered float and bit tests.
///
/// Answers 0 when the record kind (`[rec + 0x28] & 0x7c00`) is `0xc00` or
/// its byte at `+0xf17` has bit 3. Otherwise, when the object's `+0x8`
/// float is ordered-negative and `b` is 0, answers 0; else the rate getter
/// (id 1, thiscall/0, x87 float) runs. A rate that is neither ordered
/// positive nor NaN with `b` clear answers 0 (the `jb` after the compare
/// falls through only for ordered `0 >= rate`). Past that, kind-4 records
/// need two ordered-positive floats at `+0x1ef4`/`+0x1ef8` (the `jae`
/// fails on NaN too) and then pass on an ordered-positive or NaN
/// `+0x1efc`, a high (`>= 0x80`) `+0x1ef2` byte, or a zero answer from the
/// bit tester (id 2, thiscall/1); other records pass when their `+0xf84`
/// count is not positive or every row's `+0x158` float is at most 1.0.
/// Thiscall/1 (low byte), returns AL.
lf_checker_rt::export!(thiscall, rw_00a3aea0(obj: u32, b: u32) -> u32 {
    unsafe {
        const RATE_GET: u32 = 1;
        const BIT_TEST: u32 = 2;
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        let rec = core::ptr::read_unaligned(obj as *const u32);
        if core::ptr::read_unaligned((rec + 0x28) as *const u32) & 0x7C00 == 0xC00 {
            return 0;
        }
        if core::ptr::read((rec + 0xF17) as *const u8) & 8 != 0 {
            return 0;
        }
        let v0 = f32::from_bits(core::ptr::read_unaligned((obj + 8) as *const u32));
        let bb = (b & 0xFF) != 0;
        if v0 < 0.0 && !bb {
            return 0;
        }
        let t: f32 = lf_checker_rt::callee_thiscall!(RATE_GET, f32, obj);
        // jb continues on ordered-positive AND on NaN; only ordered
        // non-positive with b clear answers 0 here.
        if !(t > 0.0) && !t.is_nan() && !bb {
            return 0;
        }
        if core::ptr::read_unaligned((rec + 0x1304) as *const u32) == 4 {
            let f4 = f32::from_bits(core::ptr::read_unaligned((rec + 0x1EF4) as *const u32));
            // jae answers 0 on ordered <= AND on NaN.
            if !(f4 > 0.0) {
                return 0;
            }
            let f8 = f32::from_bits(core::ptr::read_unaligned((rec + 0x1EF8) as *const u32));
            if !(f8 > 0.0) {
                return 0;
            }
            let fc = f32::from_bits(core::ptr::read_unaligned((rec + 0x1EFC) as *const u32));
            // jb passes on ordered-positive AND on NaN.
            if !(fc <= 0.0) {
                return 1;
            }
            let b2 = core::ptr::read((rec + 0x1EF2) as *const u8);
            if (b2 as i8) <= -1 {
                return 1;
            }
            let mgr = core::ptr::read_unaligned((rec + 0xDC4) as *const u32);
            let r: u32 = lf_checker_rt::callee_thiscall!(BIT_TEST, u32, mgr, b2 as u32);
            return (r == 0) as u32;
        }
        let count = core::ptr::read_unaligned((rec + 0xF84) as *const u32) as i32;
        if count > 0 {
            let rows = core::ptr::read_unaligned((rec + 0xF80) as *const u32);
            let mut i = 0i32;
            while i < count {
                let c = rows.wrapping_add((i as u32).wrapping_mul(0x170));
                let x = f32::from_bits(core::ptr::read_unaligned((c + 0x158) as *const u32));
                if x > ONE {
                    return 0;
                }
                i += 1;
            }
        }
        1
    }
});
