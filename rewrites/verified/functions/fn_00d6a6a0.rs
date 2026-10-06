// original: 0x00d6a6a0 replay_montage_next
/// Advance the montage selector, then clear the slot flag (original 0x00D6A6A0, thiscall/0).
///
/// Calls the one-argument method (callee 1) on `this` with 1, then
/// the one-argument method (callee 2) on `this` with 2, then clears the byte
/// at `slot+0x20`, where `slot` is reached by double indirection through
/// `this+0x1c` and `+8`. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6a6a0(this_ptr: u32) -> u32 {
    unsafe {
        const LINK0_OFF: u32 = 0x1c;
        const LINK1_OFF: u32 = 8;
        const FLAG_OFF: u32 = 0x20;
        lf_checker_rt::callee_thiscall!(1, u32, this_ptr, 1);
        lf_checker_rt::callee_thiscall!(2, u32, this_ptr, 2);
        let mid =
            ((this_ptr + LINK0_OFF) as *const u32).read_unaligned();
        let slot = ((mid + LINK1_OFF) as *const u32).read_unaligned();
        ((slot + FLAG_OFF) as *mut u8).write(0);
        0
    }
});
