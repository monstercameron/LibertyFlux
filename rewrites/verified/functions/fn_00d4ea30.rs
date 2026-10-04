// original: 0x00D4EA30 CTaskSimplePickUpObject::ctor (proposed)

// Constructor of CTaskSimplePickUpObject: chains the base constructor, then initialises the members.
///
/// Runs intercepted callee 1 (the base constructor) on `this`, installs the
/// class vtable pointer, stores `arg0` at `+0x14` and `arg1` at `+0x18`, and
/// zeroes `+0x1c` and the byte at `+0x20`. When `arg0` is non-null a pointer
/// to the `+0x14` slot goes to intercepted callee 2 with `arg0` in ECX.
/// Returns `this`.
///
/// Original: 0x00D4EA30 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00d4ea30(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EE4FAC;
        const BASE_CTOR: u32 = 1;
        const ATTACH: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x14) as *mut u32).write_unaligned(arg0);
        ((this + 0x18) as *mut u32).write_unaligned(arg1);
        ((this + 0x1c) as *mut u32).write_unaligned(0);
        ((this + 0x20) as *mut u8).write(0);
        if arg0 != 0 {
            lf_checker_rt::callee_thiscall!(ATTACH, u32, arg0, this.wrapping_add(0x14));
        }
        this
    }
});
