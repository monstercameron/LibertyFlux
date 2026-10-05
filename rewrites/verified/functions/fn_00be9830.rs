// original: 0x00be9830 hkey_apply_flagged (proposed)

/// Apply the keyed handle's channels selected by the flag byte at `+0x01`.
///
/// `this` points to the record. Resolves the handle through callee 1 (cdecl,
/// one word: the key at `+0x08`) and returns quietly when it answers null or
/// the handle's key at `+0xa4` differs. Otherwise, for each set bit of the
/// flag byte at `+0x01` in ascending order: bit 0 invokes callee 2 (thiscall
/// on the handle, one word: the float bits at `+0x0c`); bit 1 invokes callee
/// 3 (thiscall on the handle, one word: the float bits at `+0x10`); bit 2
/// stages the three floats at `+0x14`/`+0x18`/`+0x1c` and passes them through
/// callee 4 (thiscall on the handle, one word: the buffer pointer, compared
/// by snapshot); bit 3 invokes callee 5 (thiscall on the handle, one word:
/// the dword at `+0x20`); bit 4 invokes callee 6 (thiscall on the handle, one
/// word: the dword at `+0x24`). No meaningful return value (`ret: none`).
///
/// Original: thiscall, no stack words, plain `ret`.
lf_checker_rt::export!(thiscall, rw_00be9830(this: u32) -> u32 {
    unsafe {
        const OFF_FLAGS: u32 = 0x01;
        const OFF_KEY: u32 = 0x08;
        const OFF_A: u32 = 0x0c;
        const OFF_B: u32 = 0x10;
        const OFF_F0: u32 = 0x14;
        const OFF_F1: u32 = 0x18;
        const OFF_F2: u32 = 0x1c;
        const OFF_D: u32 = 0x20;
        const OFF_E: u32 = 0x24;
        const H_KEY: u32 = 0xa4;
        const LOOKUP: u32 = 1;
        const APPLY_A: u32 = 2;
        const APPLY_B: u32 = 3;
        const APPLY_POS: u32 = 4;
        const APPLY_D: u32 = 5;
        const APPLY_E: u32 = 6;

        let key = ((this + OFF_KEY) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        if h == 0 || ((h + H_KEY) as *const u32).read_unaligned() != key {
            return 0;
        }
        let flags = ((this + OFF_FLAGS) as *const u8).read();
        if flags & 1 != 0 {
            let a = ((this + OFF_A) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(APPLY_A, u32, h, a);
        }
        if flags & 2 != 0 {
            let b = ((this + OFF_B) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(APPLY_B, u32, h, b);
        }
        if flags & 4 != 0 {
            let buf = [
                ((this + OFF_F0) as *const u32).read_unaligned(),
                ((this + OFF_F1) as *const u32).read_unaligned(),
                ((this + OFF_F2) as *const u32).read_unaligned(),
            ];
            lf_checker_rt::callee_thiscall!(APPLY_POS, u32, h, buf.as_ptr() as u32);
        }
        if flags & 8 != 0 {
            let d = ((this + OFF_D) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(APPLY_D, u32, h, d);
        }
        if flags & 0x10 != 0 {
            let e = ((this + OFF_E) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(APPLY_E, u32, h, e);
        }
        0
    }
});
