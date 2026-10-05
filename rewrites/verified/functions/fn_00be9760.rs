// original: 0x00be9760 hkey_apply_pos (proposed)

/// Apply a keyed handle's position triple and nothing else.
///
/// `this` points to the record. Resolves the handle through callee 1 (cdecl,
/// one word: the key at `+0x08`) and returns quietly when it answers null or
/// the handle's key at `+0xa4` differs. Otherwise stages the three floats at
/// `+0x0c`/`+0x10`/`+0x14` and passes them through callee 2 (thiscall on the
/// handle, one word: the buffer pointer, compared by snapshot). No
/// meaningful return value (`ret: none`).
///
/// Original: thiscall, no stack words, plain `ret`.
lf_checker_rt::export!(thiscall, rw_00be9760(this: u32) -> u32 {
    unsafe {
        const OFF_KEY: u32 = 0x08;
        const OFF_F0: u32 = 0x0c;
        const OFF_F1: u32 = 0x10;
        const OFF_F2: u32 = 0x14;
        const H_KEY: u32 = 0xa4;
        const LOOKUP: u32 = 1;
        const APPLY_POS: u32 = 2;

        let key = ((this + OFF_KEY) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        if h == 0 || ((h + H_KEY) as *const u32).read_unaligned() != key {
            return 0;
        }
        let buf = [
            ((this + OFF_F0) as *const u32).read_unaligned(),
            ((this + OFF_F1) as *const u32).read_unaligned(),
            ((this + OFF_F2) as *const u32).read_unaligned(),
        ];
        lf_checker_rt::callee_thiscall!(APPLY_POS, u32, h, buf.as_ptr() as u32);
        0
    }
});
