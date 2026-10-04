// original: 0x008ac690 rage::audCompressorEffect::vf1
/// Forwards `(this, a1, a2)` to the shared setter; returns its answer with the
/// low byte normalized to 0/1, preserving the upper 24 bits.
export!(thiscall, rw_008ac690(this: u32, a1: u32, a2: u32) -> u32 {
    let ans = callee_thiscall!(1, u32, this, a1, a2);
    (ans & 0xFFFFFF00) | (((ans & 0xFF) != 0) as u32)
});

