// original: 0x00D8B790 CRenderPhaseTreeImposters::update_imposter_range

/// Writes the greater of the two range endpoints at `target + 0x2d8` and
/// `target + 0x2dc` to the shared range limit. The original uses an ordered
/// comparison: when the second endpoint is not greater, including an unordered
/// NaN comparison, it stores the first endpoint's exact bits. The function
/// returns the target pointer and cleans its one stdcall argument.
lf_checker_rt::export!(stdcall, rw_00d8b790(target: u32) -> u32 {
    const FIRST_ENDPOINT: u32 = 0x2d8;
    const SECOND_ENDPOINT: u32 = 0x2dc;
    const RANGE_LIMIT: u32 = 0x011a_2944;

    unsafe {
        let first_bits = (target.wrapping_add(FIRST_ENDPOINT) as *const u32).read_unaligned();
        let second_bits = (target.wrapping_add(SECOND_ENDPOINT) as *const u32).read_unaligned();
        let first = f32::from_bits(first_bits);
        let second = f32::from_bits(second_bits);
        let selected_bits = if second > first { second_bits } else { first_bits };
        (lf_checker_rt::global::<u32>(RANGE_LIMIT)).write_unaligned(selected_bits);
    }

    target
});
