// original: 0x00A6CFA0 task_object_constructor_with_identifier

/// Runs a base constructor, initializes four local fields, and returns
/// `this`.
///
/// `identifier` is copied to `this + 0x20`. The vtable is installed at
/// `+0`, the words at `+0x14` and `+0x18` are cleared, and the 16-bit field
/// at `+0x1c` is cleared. The intercepted base call is a thiscall with no
/// stack arguments. The constructor pops its single stack argument.
lf_checker_rt::export!(thiscall, rw_00a6cfa0(this: u32, identifier: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00E9_EDBC;

        let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x14) as *mut u32).write_unaligned(0);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        ((this + 0x1C) as *mut u16).write_unaligned(0);
        ((this + 0x20) as *mut u32).write_unaligned(identifier);
        this
    }
});
