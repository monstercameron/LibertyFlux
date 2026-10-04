// original: 0x00d8e7e0 audio_pair_init_forward
/// Initialise the two slots at `this + 4` and `this + 8` from `arg`.
///
/// Both slots are set up through the same shared initialiser, which takes
/// the caller's argument as its object. Returns `this`.
lf_rs89_rt::export!(thiscall, rw_00d8e7e0(this: *mut u8, arg: u32) -> u32 {
    lf_rs89_rt::callee_thiscall!(1, u32, arg, this.wrapping_add(4) as u32);
    lf_rs89_rt::callee_thiscall!(1, u32, arg, this.wrapping_add(8) as u32);
    this as u32
});
