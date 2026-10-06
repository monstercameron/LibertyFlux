// original: 0x00c874f0 audio_entries_visit (proposed)
///
/// Applies callee 3 to every entry of the table at `this` under a lock.
/// Callee 1 takes the lock and callee 3 releases it (both thiscall with
/// no stack arguments, called with the file-global lock object
/// 0x1683290, relocated); both answer values are ignored. The head index
/// (unsigned half-word at `this+2`; 0 visits nothing) starts a chain
/// where entry `i` is `this - 0x30 + i * 0x60` and the next index is the
/// half-word at +0x58 (0 ends it); each entry is visited as
/// callee2(this, entry, arg1, arg2). Always returns 1 in al. Thiscall,
/// two stack arguments.

lf_checker_rt::export!(thiscall, rw_00c874f0(this: u32, arg1: u32, arg2: u32) -> u8 {
    unsafe {
    #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        const LOCK: u32 = 0x1683290;
        const ROW: u32 = 0x60;
        const BASE_OFF: u32 = 0x30;
        const NEXT_OFF: u32 = 0x58;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(LOCK));
        let mut idx = rd16(this.wrapping_add(2)) as u32;
        if idx != 0 {
            loop {
                let entry = this.wrapping_sub(BASE_OFF).wrapping_add(idx.wrapping_mul(ROW));
                lf_checker_rt::callee_thiscall!(2, u32, this, entry, arg1, arg2);
                idx = rd16(entry.wrapping_add(NEXT_OFF)) as u32;
                if idx == 0 {
                    break;
                }
            }
        }
        lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(LOCK));
        1
    }
});
