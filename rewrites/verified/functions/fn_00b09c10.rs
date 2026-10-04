// original: 0x00b09c10 multi_gate_mode_check
/// Decide whether the current mode switch may proceed.
///
/// Fetches the mode set and its key block; either missing aborts. When the
/// set's stored stamp matches a fresh stamp, the set must also be unlocked,
/// the primary key pair must be hot (xor above half range), the link
/// manager must be present and non-empty, and the final gate must agree. When
/// the stamps differ, any of four key pairs being hot plus the final gate
/// is enough. Returns 1 when the switch may proceed, else 0.
export!(cdecl, rw_00b09c10() -> u32 {
    unsafe {
        const MODE_OFF: usize = 0x1304;
        const STAMP_OFF: usize = 0xf50;
        const LINK_BIAS: u32 = 0x2b0;
        const MGR_STATE_OFF: usize = 0x18;
        const LOCKED_MODE: u32 = 4;
        const HOT_LIMIT: u8 = 0x7f;
        const PAIRS: [(usize, usize); 4] =
            [(0x28fc, 0x28fe), (0x290c, 0x290e), (0x26fc, 0x26fe), (0x26dc, 0x26de)];

        let set = callee_cdecl!(1, u32, 0u32);
        if set == 0 {
            return 0;
        }
        let keys = callee_cdecl!(2, u32, 0u32, 0u32);
        if keys == 0 {
            return 0;
        }
        let stamp = callee_cdecl!(3, u32,);
        let locked = *((set as usize + MODE_OFF) as *const u32) == LOCKED_MODE;
        if *((set as usize + STAMP_OFF) as *const u32) != stamp {
            let mut i = 0;
            let mut hot = false;
            while i < PAIRS.len() {
                let x = *((keys as usize + PAIRS[i].0) as *const u8);
                let y = *((keys as usize + PAIRS[i].1) as *const u8);
                if (x ^ y) > HOT_LIMIT {
                    hot = true;
                    break;
                }
                i += 1;
            }
            if !hot {
                return 0;
            }
            if (callee_cdecl!(5, u32, set, 1u32) as u8) == 0 {
                return 0;
            }
            return 1;
        }
        let mut blocked = false;
        if stamp != 0 && stamp.wrapping_add(LINK_BIAS) != 0 {
            let mgr = callee_thiscall!(4, u32, stamp.wrapping_add(LINK_BIAS));
            if mgr == 0 || *((mgr as usize + MGR_STATE_OFF) as *const u32) == 0 {
                blocked = true;
            }
        }
        if locked {
            return 0;
        }
        let x = *((keys as usize + PAIRS[0].0) as *const u8);
        let y = *((keys as usize + PAIRS[0].1) as *const u8);
        if (x ^ y) <= HOT_LIMIT {
            return 0;
        }
        if (callee_cdecl!(5, u32, set, 0u32) as u8) == 0 {
            return 0;
        }
        if blocked {
            return 0;
        }
        1
    }
});
