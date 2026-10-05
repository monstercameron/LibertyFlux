// original: 0x00888B20 stream_tag_set_value (proposed)

/// Set the stream's pending tag and value, under its lock.
///
/// When the low word of `tag` is not 0xffff, locks (callee 1) with
/// `[this+0x38]`; unless the busy count at `[this+8]` is positive, stores
/// `arg1` at `[this+0x14]`, the tag at `+0x40`, sets bytes `+0x49`/`+0x46`,
/// and stores the stamp entry's (callee 2) answer at `[this+0x2c]`.
/// Unlocks (callee 3) and returns its answer. A 0xffff tag returns 0xffff.
///
/// Original: 0x00888B20 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00888B20(this: u32, tag: u32, arg1: u32) -> u32 {
    unsafe {
        const LOCK_WORD: u32 = 0x38;
        const BUSY: u32 = 0x08;
        const VALUE: u32 = 0x14;
        const TAG: u32 = 0x40;
        const STAMP: u32 = 0x2c;
        const FLAG_A: u32 = 0x49;
        const FLAG_B: u32 = 0x46;
        const SKIP: u16 = 0xffff;
        const LOCK: u32 = 1;
        const STAMP_ENTRY: u32 = 2;
        const UNLOCK: u32 = 3;
        if tag as u16 == SKIP {
            return SKIP as u32;
        }
        let w = ((this + LOCK_WORD) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, w);
        let busy = ((this + BUSY) as *const u32).read_unaligned();
        if busy == 0 {
            ((this + VALUE) as *mut u32).write_unaligned(arg1);
            ((this + FLAG_A) as *mut u8).write(1);
            ((this + TAG) as *mut u16).write_unaligned(tag as u16);
            ((this + FLAG_B) as *mut u8).write(1);
            let s = lf_checker_rt::callee_cdecl!(STAMP_ENTRY, u32,);
            ((this + STAMP) as *mut u32).write_unaligned(s);
        }
        lf_checker_rt::callee_cdecl!(UNLOCK, u32, w)
    }
});
