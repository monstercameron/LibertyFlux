// original: 0x008B0FE0 audio_float_store_1750 (proposed)

/// Store one float argument into the object at offset `0x1750`.
///
/// `this` is the effect object, `v` the raw bits of the float. No return
/// value, no calls. Original is thiscall with one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_008B0FE0(this: u32, v: u32) -> u32 {
    const SLOT: u32 = 0x1750;
    unsafe { ((this + SLOT) as *mut u32).write_unaligned(v) }
    0
});
