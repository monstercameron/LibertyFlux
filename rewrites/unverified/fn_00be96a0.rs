// original: 0x00be96a0 hkey_apply_pos_arg2 (proposed)

/// Apply a keyed handle's position triple and second scalar.
///
/// `this` points to the record. Resolves the handle through callee 1 (cdecl,
/// one word: the key at `+0x08`) and returns quietly when it answers null or
/// the handle's key at `+0xa4` differs. Otherwise stages the three floats at
/// `+0x0c`/`+0x10`/`+0x14` and passes them through callee 2 (thiscall on the
/// handle, one word: the buffer pointer, compared by snapshot); computes the
/// target from selector byte `b0` at `handle+0x04` exactly as the sibling at
/// 0xBE9440 does (null for 0xFF, else `stride * b0 + table[b1]` with the
/// dwords at file addresses `0x115D968`/`0x115D988`); and invokes callee 3
/// (thiscall on the target, one word: the float bits at `+0x18`). No
/// meaningful return value (`ret: none`).
///
/// Original: thiscall, no stack words, plain `ret`.
lf_checker_rt::export!(thiscall, rw_00be96a0(this: u32) -> u32 {
    unsafe {
        const OFF_KEY: u32 = 0x08;
        const OFF_F0: u32 = 0x0c;
        const OFF_F1: u32 = 0x10;
        const OFF_F2: u32 = 0x14;
        const OFF_ARG2: u32 = 0x18;
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
        const APPLY_ARG2: u32 = 3;

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
        let arg2 = ((this + OFF_ARG2) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(APPLY_ARG2, u32, target, arg2);
        0
    }
});
