// original: 0x009823b0 audio_release_buffer
/// Release the buffer at `this+0x2c` and clear the buffer words.
///
/// Frees the pointer through the game's release helper (cdecl/1, stubbed),
/// then zeroes `this+0x2c` and `this+0x30`. Always returns 0.
export!(thiscall, rw_009823b0(this: u32) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, *((this.wrapping_add(0x2c)) as *const u32));
        *((this.wrapping_add(0x2c)) as *mut u32) = 0;
        *((this.wrapping_add(0x30)) as *mut u32) = 0;
        0
    }
});
