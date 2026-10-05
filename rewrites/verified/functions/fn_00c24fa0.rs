// original: 0x00c24fa0 cam_interp_release_b (proposed)
/// Release the secondary interpolator: apply the stored mode (field
/// +0x168) to the object at +0x150, drop that object's reference count
/// byte at +0x13b, and null the field. Returns the released pointer.
///
/// Original: 0x00c24fa0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00c24fa0(obj: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x150;
        const MODE_OFF: u32 = 0x168;
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
