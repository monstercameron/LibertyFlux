// original: 0x008F9260 stream_counters_reset
/// Reset the two streaming counters to zero.
///
/// Writes zero to both counter words. Takes no arguments; the original
/// returns whatever was in eax on entry, so the return value is not
/// compared (see the contract).
export!(cdecl, rw_008f9260() -> u32 {
    unsafe {
        const COUNT_A: u32 = 0x118e7d4;
        const COUNT_B: u32 = 0x118e7d8;
        *global::<u32>(COUNT_A) = 0;
        *global::<u32>(COUNT_B) = 0;
        0
    }
});
