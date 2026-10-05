// original: 0x00a4eab0 vehicle_set_ptr_logged (proposed)

/// Replace the pointer at `this`, releasing the old and acquiring the new.
///
/// A non-null old pointer is handed to the release callee (as both the
/// object in ecx and the node on the stack) before the new pointer is
/// stored; a non-null new pointer then goes to the acquire callee the same
/// way. Either call is skipped for a null pointer on its side. Thiscall,
/// one stack word, two callees, no result.
lf_checker_rt::export!(thiscall, rw_00a4eab0(this: u32, new: u32) -> u32 {
    unsafe {
        const REL: u32 = 1;
        const ACQ: u32 = 2;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let old = rd32(this);
        if old != 0 {
            lf_checker_rt::callee_thiscall!(REL, u32, old, this);
        }
        (this as *mut u32).write_unaligned(new);
        if new != 0 {
            lf_checker_rt::callee_thiscall!(ACQ, u32, new, this);
        }
        0
    }
});
