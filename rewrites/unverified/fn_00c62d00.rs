// original: 0x00C62D00 anim_player_set_blend_target

/// Stores the requested blend duration at object offset `0x5C`, then stores
/// the greater of the requested blend time and the image's minimum time at
/// offset `0x60`. The original uses an ordered floating-point compare, so a
/// value equal to or unordered with the minimum selects the minimum. This
/// thiscall method has two `f32` stack arguments and no meaningful return.
lf_checker_rt::export!(thiscall, rw_00c62d00(this: u32, duration: f32, requested_time: f32) -> () {
    const BLEND_DURATION: u32 = 0x5C;
    const BLEND_TIME: u32 = 0x60;
    const MINIMUM_TIME_VA: u32 = 0x00FE_86B4;

    unsafe {
        (this.wrapping_add(BLEND_DURATION) as *mut f32).write_unaligned(duration);
        let minimum = lf_checker_rt::global::<f32>(MINIMUM_TIME_VA).read_unaligned();
        let selected = if requested_time > minimum { requested_time } else { minimum };
        (this.wrapping_add(BLEND_TIME) as *mut f32).write_unaligned(selected);
    }
});
