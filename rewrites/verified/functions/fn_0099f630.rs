// original: 0x0099f630 SpeechObj_IsAmbientPlaying
/// Report whether either speech handle at +0x94/+0x98 is set.
///
/// Returns 1 when at least one of the two dwords is nonzero, else 0.
export!(thiscall, rw_0099f630(obj: u32) -> u32 {
    unsafe {
        let a = *((obj.wrapping_add(0x94)) as *const u32);
        let b = *((obj.wrapping_add(0x98)) as *const u32);
        ((a != 0) || (b != 0)) as u32
    }
});
