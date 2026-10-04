// original: 0x00d20850 cover_task_reinit
// Re-initialises a cover task object in place: zeroes the word block from
// +0x34 to +0x58, re-runs the two member initialisers at +0x60/+0x6C,
// releases the +0x30 slot when it is set, then sets flag bits at +0x94 and
// the default float at +0xC4. Returns nothing meaningful.
export!(thiscall, rw_00d20850(this_ptr: u32) -> u32 {
    unsafe {
        let mut off: u32 = 0x34;
        while off <= 0x58 {
            *((this_ptr.wrapping_add(off)) as *mut u32) = 0;
            off = off.wrapping_add(4);
        }
        callee_thiscall!(1, u32, this_ptr.wrapping_add(0x60));
        callee_thiscall!(2, u32, this_ptr.wrapping_add(0x6C));
        let slot = *((this_ptr.wrapping_add(0x30)) as *const u32);
        if slot != 0 {
            callee_thiscall!(3, u32, slot, this_ptr.wrapping_add(0x30));
        }
        let flags = *((this_ptr.wrapping_add(0x94)) as *const u8);
        *((this_ptr.wrapping_add(0xB0)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xC0)) as *mut u32) = 0;
        *((this_ptr.wrapping_add(0xC4)) as *mut u32) = 0x41A0_0000;
        *((this_ptr.wrapping_add(0x94)) as *mut u8) = (flags & 0xF9) | 8;
        0
    }
});
