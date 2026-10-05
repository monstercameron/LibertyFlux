// original: 0x008C6CD0 obj_ptrpair_reset
/// Release the owned buffer of a two-word streaming record, then clear it.
///
/// If the pointer at `this` is non-null it is handed to the freeing callee
/// and the slot is cleared; the counter word at `this + 4` is always
/// cleared. Returns nothing. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_008c6cd0(this: u32) -> u32 {
    unsafe {
        const FREE_CALLEE: u32 = 1;
        let slot = this as *mut u32;
        let owned = slot.read_unaligned();
        if owned != 0 {
            lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, owned);
            slot.write_unaligned(0);
        }
        ((this + 4) as *mut u32).write_unaligned(0);
        0
    }
});
