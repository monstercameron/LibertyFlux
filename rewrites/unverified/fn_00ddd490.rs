// original: 0x00DDD490 UITextField vtable install and tail
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Install this class's virtual table, clear the flag byte at `+0xC7`,
/// then tail-call the base constructor with the same `this` and return its
/// result. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00DDD490(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EFD00C;
        const FLAG: u32 = 0xC7;
        const BASE_CTOR: u32 = 1;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + FLAG) as *mut u8).write(0);
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this)
    }
});
