// original: 0x00B81850 GtaThread::vf0
/// Deleting destructor: destroy, then free when bit 0 of `flag` is set.
///
/// Always runs the destructor (callee 1); when the flag's low bit is set
/// the object is freed (callee 2). Returns `this`.
///
/// Original: 0x00B81850 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00B81850(this: u32, flag: u32) -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, this);
        if flag & 1 != 0 {
            lf_checker_rt::callee_cdecl!(2, u32, this);
        }
        this
    }
});
