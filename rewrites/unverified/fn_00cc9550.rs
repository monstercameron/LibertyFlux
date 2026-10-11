// original: 0x00CC9550 call_physics_auxiliary_member

/// Calls the receiver's thiscall helper with the receiver and the address of
/// its embedded member at `+0x100`, returning the helper's EAX unchanged.
lf_checker_rt::export!(thiscall, rw_00cc9550(this: u32) -> u32 {
    const HELPER: u32 = 1;
    const EMBEDDED_MEMBER: u32 = 0x100;

    lf_checker_rt::callee_thiscall!(
        HELPER,
        u32,
        this,
        this.wrapping_add(EMBEDDED_MEMBER)
    )
});
