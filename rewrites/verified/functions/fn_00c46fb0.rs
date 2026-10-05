// original: 0x00c46fb0 ccamscript_dtor (proposed)
/// Tear down a script camera, tail-jumping to the base destructor.
///
/// Plants the `CCamScript` vtable, calls the member destructor
/// (callee 1), then tail-calls the base destructor (callee 2) and
/// returns its result.
///
/// Original: thiscall, no stack arguments, ends in a tail jump.
lf_checker_rt::export!(thiscall, rw_00c46fb0(this: u32) -> u32 {
    const VTABLE_CCAMSCRIPT: u32 = 0x00ec9274;
    const MEMBER_DTOR: u32 = 1;
    const BASE_DTOR: u32 = 2;
    unsafe {
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_CCAMSCRIPT));
        lf_checker_rt::callee_thiscall!(MEMBER_DTOR, u32, this);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
