// original: 0x00a7e6d0 CSequenceTaskInfo::vf0
/// Scalar deleting destructor for `CSequenceTaskInfo` (vtable slot 0).
///
/// Runs the class destructor, then frees the object through the game's
/// allocator when the low flag bit is set. Returns `this`.
lf_checker_rt::export!(thiscall, rw_00a7e6d0(this: u32, flags: u32) -> u32 {
    // Restore the base vtable before running the base destructor.
    unsafe { ((this as *mut u8) as *mut u32).write_unaligned(lf_checker_rt::relocated(0xea12cc)); }
    lf_checker_rt::callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let allocator = unsafe { lf_checker_rt::global::<u32>(0x12fb1ac).read() };
        lf_checker_rt::callee_thiscall!(2, u32, allocator, this);
    }
    this
});
