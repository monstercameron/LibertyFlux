// original: 0x0050d2f0 rl_lb_ctor_Ranked_TeamDeathmatch
/// Construct a Ranked_TeamDeathmatch leaderboard info object (most-derived part).
///
/// Calls the shared base constructor (stubbed, thiscall/0) with `this`, then
/// installs this instantiation's primary vtable and secondary vtable,
/// sets the initialised flag bit, and initialises the trailing state words
/// (count/index -1, the rest zero). Returns `this`, matching the original's
/// `(an instruction of the original)`.
export!(thiscall, rw_0050d2f0(this_ptr: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this_ptr);
        let base = this_ptr as *mut u32;
        *base = relocated(0xFD17F0);
        *base.add(0x4a0 / 4) = relocated(0xFDFB9C);
        *((this_ptr + 0x5a4) as *mut u8) |= 1;
        *base.add(0x4a4 / 4) = 0xffff_ffff;
        *base.add(0x4a8 / 4) = 0;
        *base.add(0x4ac / 4) = 0;
        *base.add(0x4b0 / 4) = 0;
        *base.add(0x4b4 / 4) = 0;
        *base.add(0x4b8 / 4) = 0;
        this_ptr
    }
});
