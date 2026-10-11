// original: 0x00CB0040 CTaskComplexFollowPatrolRoute::vf20

/// If either low state bit at offset 0x28 is set, clear the word at offset
/// 0x1a and tail-dispatch this object and its stack argument through virtual
/// slot 0x4c. Otherwise return the stored word at offset 0x08. This thiscall
/// method pops one stack argument; the virtual method's return value is
/// forwarded unchanged.
lf_checker_rt::export!(thiscall, rw_00cb0040(this: u32, argument: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x28;
        const CLEARED_WORD: u32 = 0x1a;
        const STORED_RESULT: u32 = 0x08;
        if *((this + STATE) as *const u8) & 0x03 != 0 {
            ((this + CLEARED_WORD) as *mut u16).write_unaligned(0);
            lf_checker_rt::callee_thiscall!(1, u32, this, argument)
        } else {
            ((this + STORED_RESULT) as *const u32).read_unaligned()
        }
    }
});
