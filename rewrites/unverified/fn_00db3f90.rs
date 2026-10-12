// original: 0x00DB3F90 UIMouseCursor::UIMouseCursor_2

/// Install the cursor vtable, call its no-argument helper, then forward the
/// receiver to the base method that the original tail-jumps to. Return the
/// scripted base-method result unchanged.
lf_checker_rt::export!(thiscall, rw_00db3f90(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ef29c4;
        ((this + 0) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
