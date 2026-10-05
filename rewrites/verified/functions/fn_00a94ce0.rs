// original: 0x00a94ce0 stream_entry_set_flag_8000 (proposed)

/// Set bit 15 of a stream table entry's flag word and return the table base.
///
/// `this` points to a table object whose word at `+0x00` is the entry array;
/// entries are 24 bytes. The flag word of entry `idx` at `+0x0e` is ORed
/// with `0x8000`. Returns the entry array base (the loaded `[this]`).
///
/// Original: thiscall, one stack argument (index).
lf_checker_rt::export!(thiscall, rw_00a94ce0(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x00;
        const ENTRY_STRIDE: u32 = 24;
        const ENT_FLAGS: u32 = 0x0e;
        const FLAG_BIT: u16 = 0x8000;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16w(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        let base = rd32(this.wrapping_add(TABLE_BASE));
        let ent = base.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        let f = rd16w(ent.wrapping_add(ENT_FLAGS));
        wr16(ent.wrapping_add(ENT_FLAGS), f | FLAG_BIT);
        base
    }
});
