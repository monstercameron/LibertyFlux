// original: 0x00ab6c00 stream_mode_byte (proposed)

/// Read one mode byte out of an object's lane table.
///
/// Loads the lane table at `obj + 0x21C` and returns the byte at
/// `table + index + 0x67`. The low byte of the return value carries it.
///
/// Original: 0x00ab6c00 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00ab6c00(obj: u32, index: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x21C;
        const MODE_OFF: u32 = 0x67;
        let table = ((obj + TABLE_OFF) as *const u32).read_unaligned();
        ((table.wrapping_add(index).wrapping_add(MODE_OFF)) as *const u8).read() as u32
    }
});
