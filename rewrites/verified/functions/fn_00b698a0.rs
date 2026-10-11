// original: 0x00B698A0 heli_strafe_speed_scale

/// Return negative one when the object's strafe-disabled byte is set and
/// positive one otherwise. The function reads byte `+0x1f0b` and returns an
/// f32 through the x87 top-of-stack channel.
lf_checker_rt::export!(thiscall, rw_00B698A0(this: u32) -> f32 {
    unsafe {
        const STRAFE_DISABLED: u32 = 0x1f0b;
        let disabled = (this.wrapping_add(STRAFE_DISABLED) as *const u8).read_unaligned();
        if disabled != 0 { -1.0 } else { 1.0 }
    }
});
