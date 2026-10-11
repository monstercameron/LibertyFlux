// original: 0x00a2a160 player_ped_refresh_if_idle

/// Call the refresh helper for the subobject at `this+0x2b0` only when
/// byte `+0x218` is zero and byte `+0x219` is nonzero. Its one stack argument
/// is zero. The original leaves EAX dependent on entry or helper state.
lf_checker_rt::export!(thiscall, rw_a2a160(this: u32) -> () {
    let first = unsafe { ((this + 0x0218) as *const u8).read_unaligned() };
    let second = unsafe { ((this + 0x0219) as *const u8).read_unaligned() };
    if first == 0 && second != 0 {
        unsafe { lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(0x02b0), 0u32); }
    }
});
