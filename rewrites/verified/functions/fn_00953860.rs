// original: 0x00953860 tagged_spread_over_strings (proposed)

/// Spread between the largest and smallest 6-tagged values over N strings.
///
/// Reads the string count `N` from `COUNT` (signed bounds, always >= 0).
/// For each index below `N` whose flag byte at `FLAGS` is non-zero, walks
/// the bytes of the string at `TABLE`: an empty string makes no calls. Each
/// byte goes to callee 1 (one stack word; the pushed upper bytes are always
/// zero because the holder is only ever set from bytes and a zero-extended
/// count, so no mask is needed) and its answer advances the cursor. A byte
/// of 6 reads the dword 4 past it and folds it into an UNSIGNED running
/// maximum and minimum (both compares unsigned: above/below). The return
/// is maximum minus minimum, wrapping (1 when no 6 byte was seen).
///
/// Original: 0x00953860 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00953860() -> u32 {
    unsafe {
        const COUNT: u32 = 0x11F6FFB;
        const FLAGS: u32 = 0x11F6FF0;
        const TABLE: u32 = 0x11F6F7C;
        const TAG: u8 = 6;
        const STEP: u32 = 1;
        let n = (lf_checker_rt::global::<u8>(COUNT) as *const u8).read();
        let mut hi = 0u32;
        let mut lo = 0xFFFF_FFFFu32;
        if (n as i32) > 0 {
            let mut c = 0u32;
            while (c as i32) < (n as i32) {
                let f = (lf_checker_rt::relocated(FLAGS).wrapping_add(c) as *const u8).read();
                if f != 0 {
                    let base = (lf_checker_rt::relocated(TABLE)
                        .wrapping_add(c.wrapping_mul(4)) as *const u32)
                        .read();
                    let b0 = (base as *const u8).read_unaligned();
                    if b0 != 0 {
                        let mut b = b0;
                        let mut acc = 0u32;
                        loop {
                            if b == TAG {
                                let p = base.wrapping_add(acc);
                                let v =
                                    (p.wrapping_add(4) as *const u32).read_unaligned();
                                if v > hi {
                                    hi = v;
                                }
                                if v < lo {
                                    lo = v;
                                }
                            }
                            let s = lf_checker_rt::callee_cdecl!(STEP, u32, b as u32);
                            acc = acc.wrapping_add(s);
                            b = (base.wrapping_add(acc) as *const u8).read_unaligned();
                            if b == 0 {
                                break;
                            }
                        }
                    }
                }
                c += 1;
            }
        }
        hi.wrapping_sub(lo)
    }
});
