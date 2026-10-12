// original: 0x0098a520 audio_gain_block_init
/// Initializes the audio gain block's bounds and status fields.
///
/// The receiver holds a fixed collection of 32-bit values beginning at byte
/// zero, followed by one byte-sized flag at offset `0x38`. The sentinel
/// bounds are the bit pattern for negative one hundred; the remaining values
/// and the flag are cleared. The receiver address is returned unchanged.
const NEGATIVE_ONE_HUNDRED_BITS: u32 = 0xC2C8_0000;
const BOUND_WORDS: [u32; 14] = [
    0,
    NEGATIVE_ONE_HUNDRED_BITS,
    0,
    NEGATIVE_ONE_HUNDRED_BITS,
    NEGATIVE_ONE_HUNDRED_BITS,
    0,
    NEGATIVE_ONE_HUNDRED_BITS,
    0,
    NEGATIVE_ONE_HUNDRED_BITS,
    0,
    0,
    0,
    0,
    0,
];
const STATUS_FLAG: u32 = 0x38;

export!(thiscall, rw_0098a520(this_: u32) -> u32 {
    for (index, value) in BOUND_WORDS.iter().copied().enumerate() {
        unsafe {
            (this_.wrapping_add((index as u32) * 4) as *mut u32).write(value);
        }
    }
    unsafe {
        (this_.wrapping_add(STATUS_FLAG) as *mut u8).write(0);
    }
    this_
});
