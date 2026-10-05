// original: 0x00be9440 hkey_apply_pos_and_arg (proposed)

/// Apply a keyed handle's position triple and scalar argument.
///
/// `this` points to the record. Resolves the handle through callee 1 (cdecl,
/// one word: the key at `+0x08`) and returns quietly when it answers null or
/// the handle's key at `+0xa4` differs. Otherwise stages the three floats at
/// `+0x10`/`+0x14`/`+0x18` in a frame buffer and passes them through callee 2
/// (thiscall on the handle, one word: the buffer pointer; compared by
/// content snapshot, not address). Then reads selector byte `b0` at
/// `handle+0x04`: with 0xFF the target is null, otherwise it is
/// `stride * b0 + table[b1]` where `stride` is the dword at file address
/// `0x115D968` and `table[b1]` the dword at `table_base + b1 * 0x6f40 +
/// 0x6f14` (`table_base` at `0x115D988`, `b1` the byte at `handle+0x40`).
/// Invokes callee 3 (thiscall on the target, one word: the dword at
/// `+0x0c`). No meaningful return value (`ret: none`).
///
/// Original: thiscall, no stack words, plain `ret`.
lf_checker_rt::export!(thiscall, rw_00be9440(this: u32) -> u32 {
    unsafe {
        const OFF_KEY: u32 = 0x08;
        const OFF_ARG: u32 = 0x0c;
        const OFF_F0: u32 = 0x10;
        const OFF_F1: u32 = 0x14;
        const OFF_F2: u32 = 0x18;
        const H_KEY: u32 = 0xa4;
        const H_B0: u32 = 0x04;
        const H_B1: u32 = 0x40;
        const STRIDE_GLOBAL: u32 = 0x115D968;
        const TABLE_GLOBAL: u32 = 0x115D988;
        const TABLE_STRIDE: u32 = 0x6f40;
        const TABLE_BASE_OFF: u32 = 0x6f14;
        const NULL_SELECTOR: u8 = 0xFF;
        const LOOKUP: u32 = 1;
        const APPLY_POS: u32 = 2;
        const APPLY_ARG: u32 = 3;

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
        let b0 = ((h + H_B0) as *const u8).read();
        let target = if b0 == NULL_SELECTOR {
            0
        } else {
            let stride = lf_checker_rt::global::<u32>(STRIDE_GLOBAL).read_unaligned();
            let table = lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_unaligned();
            let b1 = ((h + H_B1) as *const u8).read();
            stride.wrapping_mul(b0 as u32).wrapping_add(
                (table
                    .wrapping_add((b1 as u32).wrapping_mul(TABLE_STRIDE))
                    .wrapping_add(TABLE_BASE_OFF) as *const u32)
                    .read_unaligned(),
            )
        };
        let arg = ((this + OFF_ARG) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(APPLY_ARG, u32, target, arg);
        0
    }
});
