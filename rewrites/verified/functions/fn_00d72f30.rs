// original: 0x00D72F30 MO_LINEAR

/// Dispatch a movement-mode request to the mode driver.
///
/// `this` points at the requester, whose inner object (pointer at `INNER`)
/// is resolved through the lookup callee; a null answer ends the call.
/// Otherwise the request `mode` (0..=17) selects a driver code: most modes
/// map to a fixed code, while four modes consult the resolved object:
/// mode 11 reads the state byte at `ST_MODE` (9 selects code 24, 10 selects
/// 25, anything else 18) and issues the pair directly; mode 13 reads the
/// sub-mode byte at `ST_SUB` (0/1/2 select 21/22/23, anything else falls to
/// the default); mode 15 compares the name reached through the name table
/// at file address `NAME_TABLE` (indexed by the name byte at `ST_NAME`)
/// against four literals (`PLS_NONE`, `MO_LINEAR`, `MO_BLEND`, `MO_ORBIT`,
/// selecting 28/31/32/30); mode 16 reads the variant byte at `ST_VAR`
/// (0..=3 select 33..=36). Any other request, and any unmatched sub-value,
/// takes the default: only the settle callee runs, with argument 1.
///
/// The common path issues the apply callee with (code, 1) and then the
/// settle callee with 1; all paths return 0.
///
/// Original: 0x00D72F30 (thiscall, one stack argument, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00d72f30(this: u32, mode: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x04;
        const ST_MODE: u32 = 0x00;
        const ST_NAME: u32 = 0x05;
        const ST_VAR: u32 = 0x06;
        const ST_SUB: u32 = 0x09;
        const NAME_TABLE: u32 = 0x010569D4;
        const NAME_NONE: u32 = 0x00EEB37C;
        const NAME_LINEAR: u32 = 0x00EEB388;
        const NAME_BLEND: u32 = 0x00EEB394;
        const NAME_ORBIT: u32 = 0x00EEB3A0;

        const C_LOOKUP: u32 = 1;
        const C_APPLY: u32 = 2;
        const C_SETTLE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// Byte-wise string equality against a NUL-terminated literal.
        unsafe fn streq(cand: u32, lit: u32) -> bool {
            unsafe {
                let mut i = 0u32;
                loop {
                    let a = rd8(lit + i);
                    let b = rd8(cand + i);
                    if a != b {
                        return false;
                    }
                    if a == 0 {
                        return true;
                    }
                    i += 1;
                }
            }
        }

        let inner = rd32(this + INNER);
        let obj: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, inner);
        if obj == 0 {
            return 0;
        }
        // Returns the selected code, or None for the settle-only default.
        let code: Option<u32> = match mode {
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
                let st = rd8(obj + ST_MODE);
                let c = if st == 9 {
                    0x18
                } else if st == 10 {
                    0x19
                } else {
                    0x12
                };
                lf_checker_rt::callee_thiscall!(C_APPLY, u32, inner, c, 1);
                lf_checker_rt::callee_thiscall!(C_SETTLE, u32, inner, 1);
                return 0;
            }
            12 => Some(0x13),
            13 => match rd8(obj + ST_SUB) {
                0 => Some(0x15),
                1 => Some(0x16),
                2 => Some(0x17),
                _ => None,
            },
            14 => Some(0x1b),
            15 => {
                let idx = rd8(obj + ST_NAME) as u32;
                let cand = rd32(lf_checker_rt::relocated(NAME_TABLE + idx * 4));
                if streq(cand, lf_checker_rt::relocated(NAME_NONE)) {
                    Some(0x1c)
                } else if streq(cand, lf_checker_rt::relocated(NAME_LINEAR)) {
                    Some(0x1f)
                } else if streq(cand, lf_checker_rt::relocated(NAME_BLEND)) {
                    Some(0x20)
                } else if streq(cand, lf_checker_rt::relocated(NAME_ORBIT)) {
                    Some(0x1e)
                } else {
                    None
                }
            }
            16 => match rd8(obj + ST_VAR) {
                0 => Some(0x21),
                1 => Some(0x22),
                2 => Some(0x23),
                3 => Some(0x24),
                _ => None,
            },
            17 => Some(0x1d),
            _ => None,
        };
        if let Some(c) = code {
            lf_checker_rt::callee_thiscall!(C_APPLY, u32, inner, c, 1);
        }
        lf_checker_rt::callee_thiscall!(C_SETTLE, u32, inner, 1);
        0
    }
});
