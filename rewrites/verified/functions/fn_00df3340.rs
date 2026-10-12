// original: 0x00DF3340 UITransitionViewer::UITransitionViewer

/// Run the base frame constructor with the two caller arguments in their
/// original order, then install the transition viewer's relocated vtable at
/// the start of the object and return `this`. The constructor is a 32-bit
/// thiscall with two stack arguments and callee cleanup; the object pointer and
/// vtable storage use the executable's fixed 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00df3340(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        const BASE_CONSTRUCTOR: u32 = 1;
        const OBJECT_VTABLE: u32 = 0x00f0_182c;
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this, first, second);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE));
        this
    }
});
