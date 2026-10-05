// original: 0x00945210 streaming_table_lookup (proposed)

/// Look up the streaming entry for the given index, if it is live.
///
/// Reads the live count byte. An index at or above the count returns 0.
/// Otherwise returns the pointer from the global entry table at the index.
///
/// Original: 0x00945210 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00945210(index: u32) -> u32 {
    unsafe {
        const LIVE: u32 = 0x011D74F1;
        const ENTRIES: u32 = 0x011D76AC;
        let live = lf_checker_rt::global::<u8>(LIVE).read() as u32;
        if index >= live {
            return 0;
        }
        let table = lf_checker_rt::global::<u32>(ENTRIES).read();
        ((table + index.wrapping_mul(4)) as *const u32).read_unaligned()
    }
});
