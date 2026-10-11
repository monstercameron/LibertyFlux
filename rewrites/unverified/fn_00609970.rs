// original: 0x00609970 physics_reset_joint_cache

/// Run the two scripted base reset helpers, clear the six cache words at
/// +0x240..+0x258, clear the object pointer at +0 and the state word at +0x70,
/// then return `this`. Both outgoing calls receive the object as their
/// thiscall receiver. This is a 32-bit thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00609970(this: u32) -> u32 {
    unsafe {
        const FIRST_RESET: u32 = 1;
        const SECOND_RESET: u32 = 2;
        let _ = lf_checker_rt::callee_thiscall!(FIRST_RESET, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(SECOND_RESET, u32, this);
        const ZERO_FIELDS: [u32; 8] = [0, 0x70, 0x240, 0x244, 0x248, 0x250, 0x254, 0x258];
        for offset in ZERO_FIELDS {
            ((this + offset) as *mut u32).write_unaligned(0);
        }
        this
    }
});
