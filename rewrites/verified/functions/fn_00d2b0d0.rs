// original: 0x00d2b0d0 task_release_dtor_c (proposed)
/// Destructor: stamp vtable slot `+0`, release the referenced slot at
/// `+0x34` through the release helper when non-null, then tail-jump to the
/// base destructor, forwarding `this` and its result.
///
/// Thiscall, no stack arguments. Same shape as 0x00d2b070, other vtable.
lf_checker_rt::export!(thiscall, rw_00d2b0d0(this: u32) -> u32 {
    unsafe {
        const VT0V: u32 = 0x00ee2164;
        const SLOT: u32 = 0x34;
        const RELEASE: u32 = 1;
        const TAIL: u32 = 2;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT0V));
        if ((this + SLOT) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_stdcall!(RELEASE, u32, this + SLOT);
        }
        lf_checker_rt::callee_thiscall!(TAIL, u32, this)
    }
});
