// original: 0x00dd9220 UIBasicClip::vf142

/// Mode byte of the clip.
///
/// `this` is the clip object. One byte is loaded from `MODE (+0x2f8)` and
/// returned in AL; nothing is written and no calls are made. Only the low
/// byte of the result is behaviour (the original leaves the other EAX bits
/// as they were), so the contract compares AL only.
///
/// Original: thiscall, no stack arguments, byte result in AL.
lf_checker_rt::export!(thiscall, rw_00dd9220(this: u32) -> u32 {
    const MODE: u32 = 0x2f8;
    unsafe { ((this + MODE) as *const u8).read_unaligned() as u32 }
});
