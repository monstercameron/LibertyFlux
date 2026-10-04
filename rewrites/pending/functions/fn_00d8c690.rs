// original: 0x00d8c690 audFrontendAudioEntity::vf0
/// Destroy the frontend audio entity at `this`, freeing it when flagged.
///
/// Runs the entity destructor, then releases the memory only when bit 0 of
/// the flag argument is set. Always returns `this`.
lf_rs89_rt::export!(thiscall, rw_00d8c690(this: u32, flag: u32) -> u32 {
    lf_rs89_rt::callee_thiscall!(1, u32, this);
    if flag & 1 != 0 {
        lf_rs89_rt::callee_cdecl!(2, u32, this);
    }
    this
});
