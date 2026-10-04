// original: 0x00981590 audAmbientAudioEntity::audAmbientAudioEntity_2
/// Original 0x00981590 `audAmbientAudioEntity::audAmbientAudioEntity_2`.
///
/// Second constructor phase: stamps the vtable, builds the sub-object at
/// +0x6f3c, conditionally releases the two arrays at +0x6f28/+0x6f30 (each
/// guarded by its count word), re-stamps the vtable and tail-calls the base
/// constructor, returning its result.
export!(thiscall, rw_00981590(this_: u32) -> u32 {
    unsafe { (this_ as *mut u32).write(relocated(0x00E8D9EC)); }
    callee_thiscall!(1, u32, this_.wrapping_add(0x6f3c));
    let hi = unsafe { ((this_ + 0x6f36) as *const u16).read() };
    if hi != 0 {
        let base = unsafe { ((this_ + 0x6f30) as *const u32).read() };
        callee_thiscall!(2, u32, this_.wrapping_add(0x6f30), base, hi as u32);
    }
    let lo = unsafe { ((this_ + 0x6f2e) as *const u16).read() };
    if lo != 0 {
        let base = unsafe { ((this_ + 0x6f28) as *const u32).read() };
        callee_thiscall!(3, u32, this_.wrapping_add(0x6f28), base, lo as u32);
    }
    unsafe { (this_ as *mut u32).write(relocated(0x00E83134)); }
    callee_thiscall!(4, u32, this_)
});
