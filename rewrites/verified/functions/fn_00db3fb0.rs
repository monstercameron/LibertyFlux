// original: 0x00DB3FB0 UIMouseCursor::~UIMouseCursor__deleting

/// Invoke the cursor's base destructor wrapper, conditionally release the
/// receiver when bit zero of the flags byte is set, and return the receiver.
/// The thiscall method pops its single stack argument.
lf_checker_rt::export!(thiscall, rw_00db3fb0(this: u32, destroy_flags: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        if (destroy_flags as u8) & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
        }
    }
    this
});
