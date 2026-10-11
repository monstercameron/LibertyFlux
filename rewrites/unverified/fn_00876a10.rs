// original: 0x00876A10 rage::crmtRequestExtrapolate::vf1

/// Clear the three request timing-state words at offsets +0x14, +0x1c, and
/// +0x20. The thiscall receiver is in ECX; there are no stack arguments and
/// the function has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00876a10(this: u32) -> () {
    const ELAPSED_TICKS: u32 = 0x14;
    const START_WEIGHT: u32 = 0x1c;
    const END_WEIGHT: u32 = 0x20;

    unsafe {
        ((this + ELAPSED_TICKS) as *mut u32).write_unaligned(0);
        ((this + START_WEIGHT) as *mut u32).write_unaligned(0);
        ((this + END_WEIGHT) as *mut u32).write_unaligned(0);
    }
});
