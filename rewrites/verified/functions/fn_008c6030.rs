// original: 0x008C6030 stream_slot_reset
/// Reset streaming slot `idx`: notify, release its records, clear it.
///
/// Notifies the watcher callee of the slot, releases the two records owned
/// by the slot's entry pair through the release callee, then clears the
/// slot's flag byte and zeroes its 8-byte entry. Returns nothing.
/// Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_008c6030(this: u32, idx: u32) -> u32 {
    unsafe {
        const NOTIFY_CALLEE: u32 = 1;
        const RELEASE_CALLEE: u32 = 2;
        const FLAG_BASE: u32 = 0xF0;
        const ENTRY_BASE: u32 = 0x1EC;
        const ENTRY_STRIDE: u32 = 8;
        lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, idx, 1);
        let first = this.wrapping_add(
            idx.wrapping_add(2).wrapping_mul(ENTRY_STRIDE));
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, first);
        let second = this.wrapping_add(
            idx.wrapping_add(0x10).wrapping_mul(ENTRY_STRIDE));
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, second);
        (this.wrapping_add(idx).wrapping_add(FLAG_BASE) as *mut u8).write(0);
        (this.wrapping_add(idx.wrapping_mul(ENTRY_STRIDE))
            .wrapping_add(ENTRY_BASE) as *mut u64).write_unaligned(0);
        0
    }
});
