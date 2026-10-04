// original: 0x00d72f30 replay_overlay_set_mode (proposed)

/// Select a replay-overlay mode by index and apply it through the mode pair.
///
/// `this` is the overlay object (inner state at `+0x04`); `sel` is the mode
/// index. The probe callee resolves the mode state from the inner object; a
/// zero answer returns zero at once. Otherwise `sel` picks a two-push
/// argument pair for the apply call: most indexes map to fixed pairs
/// (second word always 1), while index 11 reads a discriminator byte from
/// the state (9/10/other), index 13 a sub-mode byte (0/1/2/else, where else
/// skips the apply), index 15 a name pointer from a global table matched
/// against four known mode names (unmatched skips the apply), and index 16
/// a variant byte (0..=3, else skips the apply). Indexes above 17 and the
/// skipped cases run only the commit call. Returns the commit answer, or
/// zero on the early path.
///
/// Original: 0x00d72f30 (thiscall, one stack word). The two in-function
/// switch tables are implemented as matches; the four name comparisons are
/// byte loops with the same result as the original's unrolledstrcmp.
lf_checker_rt::export!(thiscall, rw_00d72f30(this: u32, sel: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x04;
        const NAME_TABLE_G: u32 = 0x010569d4;
        const PROBE: u32 = 1;
        const APPLY: u32 = 2;
        const COMMIT: u32 = 3;
        const PLS_NONE: &[u8] = b"PLS_NONE\0";
        const MO_LINEAR: &[u8] = b"MO_LINEAR\0";
        const MO_BLEND: &[u8] = b"MO_BLEND\0";
        const MO_ORBIT: &[u8] = b"MO_ORBIT\0";

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn name_eq(s: u32, lit: &[u8]) -> bool {
            unsafe {
                let mut i = 0u32;
                loop {
                    let a = rd8(s.wrapping_add(i));
                    let b = lit[i as usize];
                    if a != b {
                        return false;
                    }
                    if a == 0 {
                        return true;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }

        let inner = rd32(this.wrapping_add(INNER_OFF));
        let st: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, inner);
        if st == 0 {
            return 0;
        }
        let code: Option<u32> = match sel {
            0 => Some(9),
            1 => Some(0x0b),
            2 => Some(0x0a),
            3 => Some(0x0f),
            4 => Some(0x0d),
            5 => Some(0x10),
            6 | 7 | 8 => Some(0x0e),
            9 => Some(0x0c),
            10 => Some(0x11),
            11 => {
                let d = rd8(st);
                Some(if d == 9 {
                    0x18
                } else if d == 10 {
                    0x19
                } else {
                    0x12
                })
            }
            12 => Some(0x13),
            13 => {
                let b = rd8(st.wrapping_add(9));
                if b == 0 {
                    Some(0x15)
                } else if b == 1 {
                    Some(0x16)
                } else if b == 2 {
                    Some(0x17)
                } else {
                    None
                }
            }
            14 => Some(0x1b),
            15 => {
                let idx = rd8(st.wrapping_add(5)) as u32;
                let table = lf_checker_rt::global::<u32>(NAME_TABLE_G) as u32;
                let s = rd32(table.wrapping_add(idx.wrapping_mul(4)));
                if name_eq(s, PLS_NONE) {
                    Some(0x1c)
                } else if name_eq(s, MO_LINEAR) {
                    Some(0x1f)
                } else if name_eq(s, MO_BLEND) {
                    Some(0x20)
                } else if name_eq(s, MO_ORBIT) {
                    Some(0x1e)
                } else {
                    None
                }
            }
            16 => {
                let b = rd8(st.wrapping_add(6));
                match b {
                    0 => Some(0x21),
                    1 => Some(0x22),
                    2 => Some(0x23),
                    3 => Some(0x24),
                    _ => None,
                }
            }
            17 => Some(0x1d),
            _ => None,
        };
        if let Some(n) = code {
            let _: u32 = lf_checker_rt::callee_thiscall!(APPLY, u32, inner, n, 1);
        }
        lf_checker_rt::callee_thiscall!(COMMIT, u32, inner, 1)
    }
});
