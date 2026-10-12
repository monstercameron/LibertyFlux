// original: 0x00DB3F30 UIMouseCursor::UIMouseCursor

/// Forward both constructor arguments to the UIFrame constructor, then set
/// this type's relocated vtable, initialize its four word fields and enable
/// its final byte flag. ECX is the receiver; the constructor pops eight bytes.
lf_checker_rt::export!(thiscall, rw_00db3f30(this: u32, first_arg: u32, second_arg: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, first_arg, second_arg);
        const VTABLE: u32 = 0x00ef29c4;
        ((this + 0) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x1f0) as *mut u32).write_unaligned(0xffffffff);
        ((this + 0x1e0) as *mut u32).write_unaligned(0);
        ((this + 0x1e4) as *mut u32).write_unaligned(0);
        ((this + 0x1e8) as *mut u32).write_unaligned(0);
        ((this + 0x1ec) as *mut u32).write_unaligned(0);
        ((this + 0x215) as *mut u8).write(1);
    }
    this
});
