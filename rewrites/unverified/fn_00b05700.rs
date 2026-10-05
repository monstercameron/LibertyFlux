// original: 0x00b05700 reset_all_blocks
/// Bank reset: reset every lane, clear the sticky flags, hand off.
///
/// Takes the object pointer in ECX. Resets each of the twenty-four lanes
/// in turn through the lane reset entry, clears the sticky words (the
/// pair only when the sentinel already reads zero), and transfers control
/// to the shared successor with the same object pointer, returning
/// whatever that call answers.
export!(thiscall, rw_00b05700(this: u32) -> u32 {
    unsafe {
        let mut p = this;
        for _ in 0..24u32 {
            callee_thiscall!(2, u32, p);
            p = p.wrapping_add(0x54);
        }
        if *(this.wrapping_add(0x7E6) as *const u16) == 0 {
            *(this.wrapping_add(0x7E6) as *mut u16) = 0;
            *(this.wrapping_add(0x7E0) as *mut u32) = 0;
        }
        *(this.wrapping_add(0x7E4) as *mut u16) = 0;
        callee_thiscall!(1, u32, this)
    }
});
