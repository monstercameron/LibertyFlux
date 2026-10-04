// original: 0x009856a0 audEmitterAudioEntity::~audEmitterAudioEntity__deleting
/// Original 0x009856a0 `audEmitterAudioEntity::~audEmitterAudioEntity__deleting`.
///
/// Deleting destructor, same shape as 0x009815f0 for the emitter class.
export!(thiscall, rw_009856a0(this_: u32, flag: u32) -> u32 {
    callee_thiscall!(1, u32, this_);
    if flag & 1 != 0 {
        callee_cdecl!(2, u32, this_);
    }
    this_
});
