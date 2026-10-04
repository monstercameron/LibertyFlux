// original: 0x00953900 clock_span
/// Ticks elapsed between the two shared clock words.
export!(cdecl, rw_00953900() -> u32 {
    unsafe { (*global::<u32>(0x11F702C)).wrapping_sub(*global::<u32>(0x11F7028)) }
});
