// original: 0x00b05cf0 construct_with_vtable
/// Two-stage constructor: base helper, vtable, then notifier.
///
/// thiscall `(this)`: calls helper 1 (thiscall, no stack args), writes
/// the vtable address at `[this]`, calls helper 2 (thiscall, no stack
/// args), and returns `this`.
export!(thiscall, rw_00b05cf0(this: u32) -> u32 {
    const VTABLE: u32 = 0x00EA_A588;
    let _: u32 = callee_thiscall!(1, u32, this);
    unsafe {
        (this as *mut u32).write_unaligned(relocated(VTABLE));
    }
    let _: u32 = callee_thiscall!(2, u32, this);
    this
});
