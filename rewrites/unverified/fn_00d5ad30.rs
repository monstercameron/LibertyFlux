// original: 0x00d5ad30 ccam_sector_slot_register
/// Register a sector id in the first free slot of this camera's table and
/// hand the slot to the follow-up routine.
///
/// Scans the eight slot heads at `FIRST` (+0x14) stepping by `STRIDE`
/// (0x620) for the first zero dword. When all eight are taken it returns its
/// own scan pointer with the low byte cleared. Otherwise it stores the low
/// word of the stack argument at slot `+8` and tail-returns the follow-up
/// routine's answer (intercepted callee 1, argument: the slot address).
///
/// Original: thiscall, one stack argument (only its low word is used);
/// returns the follow-up answer, or the cleared scan pointer when full.
lf_checker_rt::export!(thiscall, rw_00d5ad30 (this: u32, arg: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 0x14;
        const STRIDE: u32 = 0x620;
        const SLOTS: u32 = 8;
        const ID_OFF: u32 = 8;
        const FOLLOW_UP: u32 = 1;
        let mut slot = this + FIRST;
        let mut i = 0u32;
        while (slot as *const u32).read_unaligned() != 0 {
            i += 1;
            slot += STRIDE;
            if i >= SLOTS {
                return slot & 0xFFFFFF00;
            }
        }
        ((slot + ID_OFF) as *mut u16).write_unaligned((arg & 0xFFFF) as u16);
        lf_checker_rt::callee_thiscall!(FOLLOW_UP, u32, this, slot)
    }
});
