// original: 0x00DB35B0 UIFrame::vf2

/// Invoke the layout-frame destructor first. If bit zero of the low byte of
/// the flags argument is set, then release the receiver through the second
/// helper. The method returns the receiver and pops its one stack argument.
lf_checker_rt::export!(thiscall, rw_00db35b0(this: u32, destroy_flags: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        if (destroy_flags as u8) & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
        }
    }
    this
});
