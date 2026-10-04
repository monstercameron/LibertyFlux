// original: 0x00a1abd0 cam_ctor_chain_tail (proposed)

/// Installs the vtable and runs two chained base constructors.
///
/// `this` gets the class vtable, then the first base constructor runs on
/// it, then control passes by tail call to the second base constructor on
/// the same object. Returns nothing.
///
/// Original: 0x00a1abd0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1abd0(this: u32) -> u32 {
    unsafe {
        const C1: u32 = 1;
        const C2: u32 = 2;
        const VTABLE: u32 = 0x00e9_b4a8;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(C1, u32, this);
        lf_checker_rt::callee_thiscall!(C2, u32, this);
        0
    }
});
