// original: 0x00c81dc0 flag_gated_code_select
/// Select a scenario code from flag gates, table matches and helper polls.
///
/// Returns one of 0x58, 0x55, 0x57, 0x56 or -1 (as full EAX), reading only
/// the object at `s`, three globals and four helper calls; nothing is
/// written. Bit 7 of `[s+0xf1c]` or bit 0 of `[s+0xf1d]` selects 0x58;
/// bit 5 of `[s+0xf1f]` (or a null `[s+0xf50]`, or a failed final poll)
/// selects -1. Otherwise kind checks against the globals gate the codes:
/// the 0x55 helper's nonzero answer admits a match of the kind word
/// `[s+0x2e]` against the first global, and a match against the second
/// global admits 0x55 regardless; then the 0x57 helper, a match against the
/// third global and a nonzero inspect call admit 0x57; else a 0x56 helper
/// round (probe call plus rank call) or, failing that, a second 0x56 helper
/// plus a rank call above zero admit 0x56.
///
/// Original: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_00c81dc0(s: u32) -> u32 {
    unsafe {
        const KIND_OK: u32 = 1;
        const MATCHED: u32 = 2;
        const PROBE: u32 = 3;
        const RANK: u32 = 4;
        const G1: u32 = 0x012fa230;
        const G2: u32 = 0x012f9eac;
        const G3: u32 = 0x012f9fe4;
        const NEG: u32 = 0xFFFFFFFF;
        let r8 = |o: u32| ((s + o) as *const u8).read();
        if r8(0xf1c) & 0x80 != 0 {
            return 0x58;
        }
        if r8(0xf1d) & 1 != 0 {
            return 0x58;
        }
        if r8(0xf1f) & 0x20 != 0 {
            return NEG;
        }
        let kind = ((s + 0x2e) as *const u16).read_unaligned() as i16 as i32;
        let t1: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x55);
        if (t1 & 0xFF) != 0
            && kind == lf_checker_rt::global::<i32>(G1).read_unaligned()
        {
            return 0x55;
        }
        if kind == lf_checker_rt::global::<i32>(G2).read_unaligned() {
            return 0x55;
        }
        if ((s + 0xf50) as *const u32).read_unaligned() == 0 {
            return NEG;
        }
        let mut tail56 = true;
        let t2: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x57);
        if (t2 & 0xFF) != 0
            && kind == lf_checker_rt::global::<i32>(G3).read_unaligned()
        {
            let m: u32 = lf_checker_rt::callee_thiscall!(MATCHED, u32, s, 1);
            if m != 0 {
                return 0x57;
            }
            tail56 = false;
        }
        if tail56 {
            let t3: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x56);
            if (t3 & 0xFF) != 0 {
                let p: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, 0x18, kind as u32);
                if (p & 0xFF) != 0 {
                    let r: u32 = lf_checker_rt::callee_thiscall!(RANK, u32, s);
                    if (r as i32) > 0 {
                        return 0x56;
                    }
                }
            }
        }
        let t4: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x56);
        if (t4 & 0xFF) == 0 {
            return NEG;
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(RANK, u32, s);
        if (r2 as i32) > 0 {
            0x56
        } else {
            NEG
        }
    }
});

/// Wrong version of rw_00c81dc0: the first-global match yields 0x54.
lf_checker_rt::export!(cdecl, mut_00c81dc0(s: u32) -> u32 {
    unsafe {
        const KIND_OK: u32 = 1;
        const MATCHED: u32 = 2;
        const PROBE: u32 = 3;
        const RANK: u32 = 4;
        const G1: u32 = 0x012fa230;
        const G2: u32 = 0x012f9eac;
        const G3: u32 = 0x012f9fe4;
        const NEG: u32 = 0xFFFFFFFF;
        let r8 = |o: u32| ((s + o) as *const u8).read();
        if r8(0xf1c) & 0x80 != 0 {
            return 0x58;
        }
        if r8(0xf1d) & 1 != 0 {
            return 0x58;
        }
        if r8(0xf1f) & 0x20 != 0 {
            return NEG;
        }
        let kind = ((s + 0x2e) as *const u16).read_unaligned() as i16 as i32;
        let t1: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x55);
        if (t1 & 0xFF) != 0
            && kind == lf_checker_rt::global::<i32>(G1).read_unaligned()
        {
            return 0x54; // MUTANT: 0x54 instead of 0x55.
        }
        if kind == lf_checker_rt::global::<i32>(G2).read_unaligned() {
            return 0x55;
        }
        if ((s + 0xf50) as *const u32).read_unaligned() == 0 {
            return NEG;
        }
        let mut tail56 = true;
        let t2: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x57);
        if (t2 & 0xFF) != 0
            && kind == lf_checker_rt::global::<i32>(G3).read_unaligned()
        {
            let m: u32 = lf_checker_rt::callee_thiscall!(MATCHED, u32, s, 1);
            if m != 0 {
                return 0x57;
            }
            tail56 = false;
        }
        if tail56 {
            let t3: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x56);
            if (t3 & 0xFF) != 0 {
                let p: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, 0x18, kind as u32);
                if (p & 0xFF) != 0 {
                    let r: u32 = lf_checker_rt::callee_thiscall!(RANK, u32, s);
                    if (r as i32) > 0 {
                        return 0x56;
                    }
                }
            }
        }
        let t4: u32 = lf_checker_rt::callee_cdecl!(KIND_OK, u32, 0x56);
        if (t4 & 0xFF) == 0 {
            return NEG;
        }
        let r2: u32 = lf_checker_rt::callee_thiscall!(RANK, u32, s);
        if (r2 as i32) > 0 {
            0x56
        } else {
            NEG
        }
    }
});
