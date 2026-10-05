// original: 0x00988B40 audCutsceneAudioEntity::vf2

/// Cutscene audio entity virtual slot 2: releases every live handle in the
/// table, resets the entity, then tail-calls the base slot.
///
/// When the handle table at `+TABLE_OFF` of `this` is non-null, walks its
/// `count` entries (byte at `+COUNT_OFF`, UNSIGNED bound): entry `i`'s handle
/// id is the dword at `+ENTRY_OFF + 5 * i`, resolved through the audio manager
/// at `MANAGER` (callee id 1); a nonzero handle is released with level 0
/// (callee id 2). The table word is then cleared. Always afterwards the reset
/// callee (id 3) runs with argument 1, and the base slot (id 4) is tail-called
/// with `this`, whose answer is returned.
/// Original: thiscall, no stack words, ends in a jump.
lf_checker_rt::export!(thiscall, rw_00988B40(this: u32) -> u32 {
    const TABLE_OFF: u32 = 0x94;
    const COUNT_OFF: u32 = 0x0a;
    const ENTRY_OFF: u32 = 0x0b;
    const ENTRY_STRIDE: u32 = 5;
    const MANAGER: u32 = 0x115d9a0;
    const RESOLVE: u32 = 1;
    const RELEASE: u32 = 2;
    const RESET: u32 = 3;
    const TAIL_BASE: u32 = 4;
    unsafe {
        let mgr = lf_checker_rt::relocated(MANAGER);
        let tbl = ((this + TABLE_OFF) as *const u32).read_unaligned();
        if tbl != 0 {
            let count = ((tbl + COUNT_OFF) as *const u8).read() as u32;
            if count != 0 {
                let mut i = 0u32;
                let mut off = 0u32;
                while i < count {
                    let id = ((tbl + ENTRY_OFF + off) as *const u32)
                        .read_unaligned();
                    let h: u32 =
                        lf_checker_rt::callee_thiscall!(RESOLVE, u32, mgr, id);
                    if h != 0 {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(RELEASE, u32, h, 0);
                    }
                    i = i.wrapping_add(1);
                    off = off.wrapping_add(ENTRY_STRIDE);
                }
            }
            ((this + TABLE_OFF) as *mut u32).write_unaligned(0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(RESET, u32, this, 1);
        lf_checker_rt::callee_thiscall!(TAIL_BASE, u32, this)
    }
});
