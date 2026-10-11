// original: 0x0060A0C0 physics_is_supported_joint_mode

/// Return one in AL only when the 32-bit mode at `this + 0x14` is exactly 1
/// or 3. The original leaves the value's upper 24 bits in EAX, so the rewrite
/// preserves them as well. This is a register-only 32-bit thiscall.
lf_checker_rt::export!(thiscall, rw_0060A0C0(this: u32) -> u32 {
    unsafe {
        let mode = ((this + 0x14) as *const u32).read_unaligned();
        (mode & 0xffff_ff00) | u32::from(mode == 1 || mode == 3)
    }
});
