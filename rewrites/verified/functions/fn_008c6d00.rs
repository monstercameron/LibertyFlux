// original: 0x008C6D00 stream_bulk_reset
/// Reset the whole streaming control block: notify, release all, clear.
///
/// Runs the notify callee once, releases the three header records and both
/// records of each of the 14 slots through the release callee, clears the
/// two header flags and every slot flag, and zeroes every 8-byte slot
/// entry. Returns nothing. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_008c6d00(this: u32) -> u32 {
    unsafe {
        const NOTIFY_CALLEE: u32 = 1;
        const RELEASE_CALLEE: u32 = 2;
        const SLOT_COUNT: u32 = 14;
        const FLAG_BASE: u32 = 0xF0;
        const ENTRY_OFF: u32 = 0x16C;
        lf_checker_rt::callee_thiscall!(NOTIFY_CALLEE, u32, this);
        ((this + 0x262) as *mut u8).write(0);
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, this);
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, this + 8);
        lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, this + 0x264);
        ((this + 0x261) as *mut u8).write(0);
        let mut slot = this.wrapping_add(0x80);
        let mut i: u32 = 0;
        while i < SLOT_COUNT {
            lf_checker_rt::callee_thiscall!(
                RELEASE_CALLEE, u32, slot.wrapping_sub(0x70));
            lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, slot);
            (this.wrapping_add(i).wrapping_add(FLAG_BASE) as *mut u8)
                .write(0);
            ((slot + ENTRY_OFF) as *mut u64).write_unaligned(0);
            i += 1;
            slot = slot.wrapping_add(8);
        }
        0
    }
});
