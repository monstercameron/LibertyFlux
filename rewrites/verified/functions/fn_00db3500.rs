// original: 0x00DB3500 UIFrame::UIFrame

/// Initialize a UI frame by forwarding its two constructor arguments to the
/// layout-frame constructor, then establish this type's vtable and empty child
/// list. The vtable pointer is relocated from the image; ECX is the receiver
/// and the callee pops both stack arguments.
lf_checker_rt::export!(thiscall, rw_00db3500(this: u32, first_arg: u32, second_arg: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, first_arg, second_arg);
        const VTABLE: u32 = 0x00ef251c;
        const CHILD_ARRAY: u32 = 0x1d4;
        const CHILD_COUNT: u32 = 0x1d8;
        const CHILD_END: u32 = 0x1dc;
        const ENABLED: u32 = 0xca;

        ((this + 0) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + CHILD_ARRAY) as *mut u32).write_unaligned(0);
        ((this + CHILD_COUNT) as *mut u32).write_unaligned(0);
        ((this + CHILD_END) as *mut u32).write_unaligned(0);
        ((this + ENABLED) as *mut u8).write(1);
    }
    this
});
