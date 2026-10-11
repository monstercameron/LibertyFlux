// original: 0x00A6CD50 CTaskComplexPlayerInCover::CTaskComplexPlayerInCover

/// Initializes the task's base object and its two local fields.
///
/// `this` is the task object; `mode` is copied as one byte to `+0x18`. The
/// vtable is installed at `+0`, the reference-like field at `+0x14` is
/// cleared, and the byte at `+0x19` is cleared. The base constructor is an
/// intercepted thiscall with no stack arguments. The constructor returns
/// `this` and pops its one stack argument.
lf_checker_rt::export!(thiscall, rw_00a6cd50(this: u32, mode: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00E9_F00C;

        let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x14) as *mut u32).write_unaligned(0);
        ((this + 0x18) as *mut u8).write(mode as u8);
        ((this + 0x19) as *mut u8).write(0);
        this
    }
});
