// original: 0x00DE3FE0 UIRowLayout::UIRowLayout

/// Initialize a row-layout object by forwarding its two constructor words to
/// the layout-manager constructor, then installing the relocated row-layout
/// vtable and setting the state words at `+0x1e4`, `+0x1e8`, `+0x1ec`, and
/// `+0x1f4`. Return the object pointer. This is a `thiscall` constructor whose
/// caller-provided two stack arguments are removed on return.
lf_checker_rt::export!(thiscall, rw_00DE3FE0(this: u32, arg0: u32, arg1: u32) -> u32 {
    const MANAGER_CONSTRUCTOR: u32 = 1;
    const VTABLE_WORD: u32 = 0;
    const STATE_ZERO: u32 = 0x1e4;
    const STATE_TWO: u32 = 0x1e8;
    const STATE_FLAGS: u32 = 0x1ec;
    const STATE_FINAL: u32 = 0x1f4;
    const ROW_LAYOUT_VTABLE: u32 = 0x00EFE09C;
    let _ = lf_checker_rt::callee_thiscall!(MANAGER_CONSTRUCTOR, u32, this, arg0, arg1);
    unsafe {
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(ROW_LAYOUT_VTABLE));
        (this.wrapping_add(STATE_ZERO) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_TWO) as *mut u32).write_unaligned(2);
        (this.wrapping_add(STATE_FLAGS) as *mut u32).write_unaligned(0x01000000);
        (this.wrapping_add(STATE_FINAL) as *mut u32).write_unaligned(0);
    }
    this
});
