// original: 0x00BE91F0 handle_release_checked (proposed)
/// Release a looked-up handle when its back-pointer matches the key.
///
/// `obj` points at the key word at `+0x08`. The probe callee (cdecl,
/// key) selects the path by its LOW byte: zero takes the alternate
/// path, any other value the main path. On the main path the lookup
/// callee (cdecl, key) must return nonzero, its target's head word must
/// be nonzero, and the back-pointer at head `+0xa4` must equal the key;
/// then the release callee runs (thiscall on the head with 0) and the
/// handle word is zeroed. On the alternate path the other lookup callee
/// (cdecl, key) must return nonzero (full-word test) with its
/// back-pointer at `+0xa4` equal to the key, and the release callee runs
/// on it instead. Any failed test ends the function quietly. No result.
///
/// Original: 0x00BE91F0 (thiscall, no stack words). Note the asymmetric
/// tests: the probe checks only al, the alternate lookup all of eax.
lf_checker_rt::export!(thiscall, rw_00BE91F0(obj: u32) -> u32 {
    unsafe {
        const KEY: u32 = 0x08;
        const BACK: u32 = 0xa4;
        const PROBE: u32 = 1;
        const LOOKUP: u32 = 2;
        const RELEASE: u32 = 3;
        const ALT_LOOKUP: u32 = 4;
        let key = ((obj + KEY) as *const u32).read_unaligned();
        let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32, key);
        if (probe as u8) != 0 {
            let h: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
            if h == 0 {
                return 0;
            }
            let head = (h as *const u32).read_unaligned();
            if head == 0 {
                return 0;
            }
            if ((head + BACK) as *const u32).read_unaligned() != key {
                return 0;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, head, 0);
            (h as *mut u32).write_unaligned(0);
            return 0;
        }
        let h: u32 = lf_checker_rt::callee_cdecl!(ALT_LOOKUP, u32, key);
        if h == 0 {
            return 0;
        }
        if ((h + BACK) as *const u32).read_unaligned() != key {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, h, 0);
        0
    }
});

