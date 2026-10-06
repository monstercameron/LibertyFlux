// original: 0x00d69df0 replay_rollover_set_kind
/// Set the rollover message kind, reformatting on change (original 0x00D69DF0,
/// thiscall/1).
///
/// Compares the low word of `kind` against the word at `this+0x1e` for
/// equality; when they differ, calls the three-argument formatter (callee 1)
/// with (`this+0x20`, the table word at file VA 0x01056718 indexed by the
/// kind, 0x100), clears the dirty byte at `this+0x11f` and stores the new
/// kind. Equal kinds do nothing. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d69df0(this_ptr: u32, kind: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x1e;
        const BUF_OFF: u32 = 0x20;
        const DIRTY_OFF: u32 = 0x11f;
        const TABLE: u32 = 0x01056718;
        const BUF_CAP: u32 = 0x100;
        let di = (kind & 0xFFFF) as u16;
        let cur = ((this_ptr + KIND_OFF) as *const u16).read_unaligned();
        if di != cur {
            let idx = di as u32;
            let word = ((lf_checker_rt::relocated(TABLE) + idx * 4)
                as *const u32)
                .read_unaligned();
            lf_checker_rt::callee_cdecl!(1, u32, this_ptr + BUF_OFF, word,
                BUF_CAP);
            ((this_ptr + DIRTY_OFF) as *mut u8).write(0);
            ((this_ptr + KIND_OFF) as *mut u16).write_unaligned(di);
        }
        0
    }
});
