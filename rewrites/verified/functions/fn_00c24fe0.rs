// original: 0x00c24fe0 cam_interp_release_a (proposed)
/// Release the primary interpolator: the mirror of the secondary release
/// with the mode from +0x164 and the object from +0x14c.
///
/// Original: 0x00c24fe0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c24fe0(obj: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x14c;
        const MODE_OFF: u32 = 0x164;
        const REFCOUNT_OFF: u32 = 0x13b;
        let mode = (obj.wrapping_add(MODE_OFF) as *const u32).read_unaligned();
        let arg0 = (obj.wrapping_add(TARGET_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_stdcall!(1, u32, arg0, mode);
        let target = (obj.wrapping_add(TARGET_OFF) as *const u32).read_unaligned();
        let p = target.wrapping_add(REFCOUNT_OFF) as *mut u8;
        p.write(p.read().wrapping_sub(1));
        (obj.wrapping_add(TARGET_OFF) as *mut u32).write_unaligned(0);
        target
    }
});
