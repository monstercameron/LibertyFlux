// original: 0x009815f0 audAmbientAudioEntity::~audAmbientAudioEntity__deleting
/// Original 0x009815f0 `audAmbientAudioEntity::~audAmbientAudioEntity__deleting`.
///
/// Deleting destructor: runs the destructor, then frees `this` with the
/// scalar-deleting convention when the low bit of `flag` is set. Returns `this`.
export!(thiscall, rw_009815f0(this_: u32, flag: u32) -> u32 {
    callee_thiscall!(1, u32, this_);
    if flag & 1 != 0 {
        callee_cdecl!(2, u32, this_);
    }
    this_
});
