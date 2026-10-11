// original: 0x00B6CF30 heli_motion_fields_are_zero

/// Reject the special type bits in `this + 0x28`, then return true if any of
/// the four f32 words at `+0x1ec0`, `+0x1eb4`, `+0x1eb8` or `+0x1ebc` is
/// nonzero or NaN. Both signed zero encodings compare equal to zero. When all
/// four compare equal to zero, call the no-argument thiscall helper and return
/// whether its EAX result is zero; the public boolean is in AL.
lf_checker_rt::export!(thiscall, rw_00B6CF30(this: u32) -> u32 {
    unsafe {
        const TYPE_FLAGS: u32 = 0x28;
        const SPECIAL_MASK: u32 = 0x7c00;
        const SPECIAL_VALUE: u32 = 0x0c00;
        const MOTION_FIELDS: [u32; 4] = [0x1ec0, 0x1eb4, 0x1eb8, 0x1ebc];
        const IDLE_HELPER: u32 = 1;

        let flags = (this.wrapping_add(TYPE_FLAGS) as *const u32).read_unaligned();
        if flags & SPECIAL_MASK == SPECIAL_VALUE {
            return 0;
        }
        for offset in MOTION_FIELDS {
            let bits = (this.wrapping_add(offset) as *const u32).read_unaligned();
            if bits & 0x7fff_ffff != 0 {
                return 1;
            }
        }
        let helper_result = lf_checker_rt::callee_thiscall!(IDLE_HELPER, u32, this);
        u32::from(helper_result == 0)
    }
});
