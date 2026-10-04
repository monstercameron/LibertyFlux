// original: 0x008aad70 audio_subobj_reinit
/// Reinitialise the sub-object through the helper and return `this`.
///
/// Forwards `this` to the helper (thiscall/0, stubbed by the checker) and
/// returns `this` unchanged.
export!(thiscall, rw_008aad70(this: *mut u8) -> u32 {
    callee_thiscall!(1, u32, this as u32);
    this as u32
});
