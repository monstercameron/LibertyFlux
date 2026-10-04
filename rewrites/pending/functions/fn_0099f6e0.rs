// original: 0x0099f6e0 NativeImpl_IS_ANY_SPEECH_PLAYING
/// Report whether either speech handle at +0xB0/+0xB4 is set.
///
/// Same shape as the +0x94/+0x98 check at a different member pair.
export!(thiscall, rw_0099f6e0(obj: u32) -> u32 {
    unsafe {
        let a = *((obj.wrapping_add(0xB0)) as *const u32);
        let b = *((obj.wrapping_add(0xB4)) as *const u32);
        ((a != 0) || (b != 0)) as u32
    }
});
