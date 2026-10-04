// original: 0x00967120 timing_ids_reset
/// Invalidate three global timing identifiers by writing -1 to each.
///
/// The three globals are file VAs `0x0106283C`, `0x0106B6DC` and
/// `0x0110B604`. Takes no arguments; the original leaves `eax` untouched,
/// so there is no return channel (`ret: none`).
///
/// Original: 0x00967120 (cdecl, no stack words).

export!(cdecl, rw_00967120() -> u32 {
    unsafe {
        *global::<u32>(0x0106283C) = 0xFFFF_FFFF;
        *global::<u32>(0x0106B6DC) = 0xFFFF_FFFF;
        *global::<u32>(0x0110B604) = 0xFFFF_FFFF;
        0
    }
});
