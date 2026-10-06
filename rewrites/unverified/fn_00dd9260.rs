// original: 0x00dd9260 UIBasicClip::vf139

/// Sink word of the clip.
///
/// `this` is the clip object. One word is loaded from `SINK (+0x1ec)` and
/// returned in EAX; nothing is written and no calls are made.
///
/// Original: thiscall, no stack arguments, word result in EAX.
lf_checker_rt::export!(thiscall, rw_00dd9260(this: u32) -> u32 {
    const SINK: u32 = 0x1ec;
    unsafe { ((this + SINK) as *const u32).read_unaligned() }
});
