// original: 0x008B1080 audio_pair_store_17e4 (proposed)

/// Store two argument words into the object at offsets `0x17e4` and `0x17e8`.
///
/// `this` is the effect object. No return value, no calls. Original is
/// thiscall with two stack words (the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_008B1080(this: u32, a: u32, b: u32) -> u32 {
    const SLOT_A: u32 = 0x17e4;
    const SLOT_B: u32 = 0x17e8;
    unsafe {
        ((this + SLOT_A) as *mut u32).write_unaligned(a);
        ((this + SLOT_B) as *mut u32).write_unaligned(b);
    }
    0
});
