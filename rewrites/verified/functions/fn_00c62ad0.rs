// original: 0x00c62ad0 anim_player_stop
/// Player stop: detaches playback state and clears the tag word.
///
/// Runs the detach helper, then zeroes the tag word at `+0x44`. Returns 0.
export!(thiscall, rw_00c62ad0(this: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(0, u32, this);
        *((this + 0x44) as *mut u16) = 0;
        0
    }
});
