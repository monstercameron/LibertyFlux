// original: 0x00DDF680 UITextField destructor head
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Begin destruction: install this class's virtual table, run the member
/// cleanup, then tail-call the base destructor with the same `this` and
/// return its result. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00DDF680(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EFD374;
        const MEMBER_CLEANUP: u32 = 1;
        const BASE_DTOR: u32 = 2;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(MEMBER_CLEANUP, u32, this);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
