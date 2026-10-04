// original: 0x00e64090 audio_buffer_fill_0080
/// Fill the audio scratch buffer with 0x0080 words (silence pattern).
///
/// Writes 0x3840 little-endian 16-bit words of value 0x0080 over the range
/// 0x1218558..0x121F5D8. The original does this as 0x960 iterations of three
/// `0x0080080` dword stores 12 bytes apart, which is the same byte pattern
/// (80 00 repeated). Returns the end pointer, matching the original's EAX.
export!(cdecl, rw_00e64090() -> u32 {
    unsafe {
        let dst = global::<u16>(0x1218558);
        let mut i = 0usize;
        while i < 0x3840 {
            *dst.add(i) = 0x0080;
            i += 1;
        }
        relocated(0x121855B).wrapping_add(0x7080)
    }
});
