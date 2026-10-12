// original: 0x00DF3360 UITransitionViewer::UITransitionViewer_2

/// Install the transition viewer vtable at the start of the object, invoke
/// the clip-specific initialization hook, then tail-call the frame routine
/// that destroys any existing child controls. The virtual table pointer is
/// relocated from the mapped image, and both calls preserve the original
/// `this` receiver and order.
lf_checker_rt::export!(thiscall, rw_00df3360(this: u32) -> u32 {
    unsafe {
        const OBJECT_VTABLE: u32 = 0x00f0_182c;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this)
    }
});
