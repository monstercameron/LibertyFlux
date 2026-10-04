// original: 0x00a1ab40 cam_ctor_full_init (proposed)

/// Fully constructs the camera object: base, vtable, member, derived.
///
/// Runs the base constructor on `this`, installs the class vtable,
/// constructs the member at `this + MEMBER_OFF`, zeroes the two words at
/// `+ZERO_OFF`, runs the derived constructor on `this`, and returns
/// `this`.
///
/// Original: 0x00a1ab40 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1ab40(this: u32) -> u32 {
    unsafe {
        const C_BASE: u32 = 1;
        const C_MEMBER: u32 = 2;
        const C_DERIVED: u32 = 3;
        const VTABLE: u32 = 0x00e9_b4a8;
        const MEMBER_OFF: u32 = 0x140;
        const ZERO_OFF: u32 = 0x1e0;
        lf_checker_rt::callee_thiscall!(C_BASE, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(C_MEMBER, u32, this + MEMBER_OFF);
        ((this + ZERO_OFF) as *mut u32).write_unaligned(0);
        ((this + ZERO_OFF + 4) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(C_DERIVED, u32, this);
        this
    }
});
