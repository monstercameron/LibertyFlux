// original: 0x00981550 audAmbientAudioEntity::audAmbientAudioEntity
/// Original 0x00981550 `audAmbientAudioEntity::audAmbientAudioEntity`.
///
/// Constructor: runs the base constructor, stamps the vtable pointer, zeroes
/// the four header words at +0x6f28, then constructs the sub-object at
/// +0x6f3c. Returns `this`.
export!(thiscall, rw_00981550(this_: u32) -> u32 {
    callee_thiscall!(1, u32, this_);
    unsafe {
        let obj = this_ as *mut u32;
        obj.write(relocated(0x00E8D9EC));
        for i in 0..4u32 {
            (obj as *mut u8).add(0x6f28 + (i * 4) as usize).cast::<u32>().write(0);
        }
    }
    callee_thiscall!(2, u32, this_.wrapping_add(0x6f3c));
    this_
});
