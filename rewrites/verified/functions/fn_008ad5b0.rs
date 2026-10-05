// original: 0x008AD5B0 audio_param_lookup (proposed)

/// Resolve a parameter handle, then dispatch it through the parameter table.
///
/// Calls the resolver (`0x40ba60`, cdecl: the argument word, then 0) and
/// passes its result to the dispatcher (`0x8ad480`, thiscall on `this` with
/// one stack word), returning the dispatcher's value. Original is thiscall
/// with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008AD5B0(this: u32, arg: u32) -> u32 {
    const RESOLVE: u32 = 1;
    const DISPATCH: u32 = 2;
    unsafe {
        let h: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, arg, 0u32);
        lf_checker_rt::callee_thiscall!(DISPATCH, u32, this, h)
    }
});
