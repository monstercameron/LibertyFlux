// original: 0x009a29f0 mode_byte_store
/// Store a mode byte at +0xC1, restoring the default dword at +0xC4 on zero.
///
/// Only the low byte of the argument word is significant. When it is zero,
/// the dword at +0xC4 is reloaded from its global default; the low byte of
/// the return matches the original's AL on both paths.
export!(thiscall, rw_009a29f0(obj: u32, w: u32) -> u32 {
    unsafe {
        let v = (w & 0xFF) as u8;
        *((obj.wrapping_add(0xC1)) as *mut u8) = v;
        if v == 0 {
            let d = *global::<u32>(0x11735B4);
            *((obj.wrapping_add(0xC4)) as *mut u32) = d;
            d
        } else {
            v as u32
        }
    }
});
