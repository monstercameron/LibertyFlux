// original: 0x00a94b30 stream_entry_kind_flag (proposed)

/// Kind flag byte of stream table entry `idx`, with scaled-index residue.
///
/// The kind byte at `+0x04` of the 24-byte entry is scaled by 160 and used
/// to index a global byte table; the returned `eax` keeps the scaled index
/// in its upper 24 bits with the table byte in `al`. Pure leaf.
///
/// Original: thiscall, one stack argument (index).
lf_checker_rt::export!(thiscall, rw_00a94b30(this: u32, idx: u32) -> u32 {
    unsafe {
        const TABLE_BASE: u32 = 0x00;
        const ENTRY_STRIDE: u32 = 24;
        const ENT_KIND: u32 = 0x04;
        const KIND_STRIDE: u32 = 160;
        const KIND_TABLE: u32 = 0x012fb457;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        let base = rd32(this.wrapping_add(TABLE_BASE));
        let ent = base.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE));
        let scaled = (rd8(ent.wrapping_add(ENT_KIND)) as u32).wrapping_mul(KIND_STRIDE);
        let flag = rd8(lf_checker_rt::relocated(KIND_TABLE).wrapping_add(scaled));
        // The black_box works around a backend miscompile on i686: without
        // it, (masked | indexed-load) folds to just the load (see report).
        (core::hint::black_box(scaled) & 0xffff_ff00) | (flag as u32)
    }
});
