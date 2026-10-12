// original: 0x00881010 audio_channel_index_from_pointer

/// Return the signed element index for an address in a 100-byte channel array.
///
/// The receiver's base address is the word at offset `0x58`. Subtract it from
/// the input with 32-bit wrapping, reinterpret the difference as signed, and
/// divide by the signed positive stride using truncation toward zero.
lf_checker_rt::export!(thiscall, rw_00881010(this: u32, address: u32) -> u32 {
    unsafe {
        const CHANNEL_BASE_OFFSET: u32 = 0x58;
        const CHANNEL_STRIDE: i32 = 100;
        let base = (this.wrapping_add(CHANNEL_BASE_OFFSET) as *const u32).read_unaligned();
        ((address.wrapping_sub(base) as i32) / CHANNEL_STRIDE) as u32
    }
});
