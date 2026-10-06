// original: 0x00c87840 audio_entry_acquire (proposed)
///
/// Finds or opens the entry for `key`. When the low byte of `flags` is
/// non-zero, callee 1 (the entry search, thiscall) runs first and its
/// non-null result is returned as is. Otherwise the head index (unsigned
/// half-word at `this+0`; 0 means empty, return null) selects entry
/// `this - 0x30 + idx * 0x60`, callee 2 (thiscall) opens it, and the
/// entry address is returned when that call's result is non-null, else
/// null. Thiscall, two stack arguments (key, flags).

lf_checker_rt::export!(thiscall, rw_00c87840(this: u32, key: u32, flags: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        const ROW: u32 = 0x60;
        const BASE_OFF: u32 = 0x30;
        if flags & 0xff != 0 {
            let found = lf_checker_rt::callee_thiscall!(1, u32, this, key);
            if found != 0 {
                return found;
            }
        }
        let idx = rd16(this.wrapping_add(0)) as u32;
        if idx == 0 {
            return 0;
        }
        let entry = this.wrapping_sub(BASE_OFF).wrapping_add(idx.wrapping_mul(ROW));
        let opened = lf_checker_rt::callee_thiscall!(2, u32, entry, key);
        if opened == 0 {
            return 0;
        }
        entry
    }
});
