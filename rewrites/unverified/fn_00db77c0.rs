// original: 0x00DB77C0 UIMusicViewer::UIMusicViewer

/// Forward both constructor arguments to the UIFrame constructor, set the
/// viewer's relocated vtable and return the receiver. ECX is the object pointer
/// and this constructor removes both stack arguments.
lf_checker_rt::export!(thiscall, rw_00db77c0(this: u32, first_arg: u32, second_arg: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, first_arg, second_arg);
        const VTABLE: u32 = 0x00ef36f4;
        ((this + 0) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
    }
    this
});
