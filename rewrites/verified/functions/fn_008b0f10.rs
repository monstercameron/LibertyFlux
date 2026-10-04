// original: 0x008b0f10 audio_flag_store_17e0
/// Stores the low byte of the argument at `this+0x17e0`.
export!(thiscall, rw_008b0f10(this: u32, v: u32) -> () {
    unsafe {
        ((this.wrapping_add(0x17e0)) as *mut u8).write_unaligned((v & 0xFF) as u8);
    }
});
