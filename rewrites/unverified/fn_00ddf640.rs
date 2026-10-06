// original: 0x00DDF640 UITextField constructor
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Construct a text field: run the base constructor with (`a`, `b`), then
/// install this class's virtual table and clear the child slots at `+0x1E0`,
/// `+0x1E4` and `+0x1E8`. Returns `this`.
/// Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_00DDF640(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EFD374;
        const BASE_CTOR: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, a, b);
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x1E0) as *mut u32).write_unaligned(0);
        ((this + 0x1E4) as *mut u32).write_unaligned(0);
        ((this + 0x1E8) as *mut u32).write_unaligned(0);
        this
    }
});
