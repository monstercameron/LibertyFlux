// original: 0x009a5490 audScriptAudioEntity::vf0
/// Original 0x009a5490 `audScriptAudioEntity::vf0`: deleting destructor.
///
/// Tears down the sub-object at +0x2be0, stamps the base vtable, runs the
/// base destructor, and frees `this` when the low bit of `flag` is set.
/// Returns `this`.
export!(thiscall, rw_009a5490(this_: u32, flag: u32) -> u32 {
    callee_thiscall!(1, u32, this_.wrapping_add(0x2be0));
    unsafe { (this_ as *mut u32).write(relocated(0x00E83134)) };
    callee_thiscall!(2, u32, this_);
    if flag & 1 != 0 {
        callee_cdecl!(3, u32, this_);
    }
    this_
});
