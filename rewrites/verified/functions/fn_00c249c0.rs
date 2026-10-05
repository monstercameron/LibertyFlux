// original: 0x00c249c0 cam_interp_init (proposed)
/// Initialise a camera-interpolator object: zero the pointer and parameter
/// fields, set the state word at +0x148 to -1, the active flag at +0x140,
/// the enabled flag at +0x160, then run the secondary initialiser. Returns
/// the callee's answer with its low byte forced to 1 (the original sets
/// only `al`, leaving the upper bytes from the call).
///
/// Original: 0x00c249c0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c249c0(obj: u32) -> u32 {
    unsafe {
        for off in [0x154u32, 0x158, 0x14c, 0x150, 0x144] {
            (obj.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        (obj.wrapping_add(0x148) as *mut u32).write_unaligned(0xffff_ffff);
        (obj.wrapping_add(0x140) as *mut u8).write(1);
        for off in [0x170u32, 0x174, 0x178, 0x17c, 0x180, 0x184, 0x188, 0x18c, 0x190, 0x194] {
            (obj.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        (obj.wrapping_add(0x1e8) as *mut u8).write(0);
        (obj.wrapping_add(0x15c) as *mut u32).write_unaligned(0);
        (obj.wrapping_add(0x160) as *mut u32).write_unaligned(1);
        for off in [0x164u32, 0x168, 0x16c] {
            (obj.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj);
        (r & 0xffff_ff00) | 1
    }
});
