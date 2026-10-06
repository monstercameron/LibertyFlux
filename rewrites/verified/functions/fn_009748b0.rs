// original: 0x009748b0 audGtaOcclusionGroup::vf2

/// Store a value into the occlusion group's field at +0xA4.
///
/// `this` is the group object. The original leaves the accumulator
/// untouched, so no return channel is compared.
/// Original: 0x009748B0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_009748b0(this: u32, value: u32) -> u32 {
    unsafe {
        const FIELD: u32 = 0xA4;
        ((this.wrapping_add(FIELD)) as *mut u32).write_unaligned(value);
        0
    }
});
