// original: 0x00e630f0 fill_table_minus_one_5000
/// Fill a flat table of 5000 dwords with -1 (0xFFFFFFFF) and return -1, which
/// is also what the original leaves in EAX.
export!(cdecl, rw_00e630f0() -> u32 {
    const WORDS: usize = 0x1388;
    const FILL: u32 = 0xFFFF_FFFF;
    unsafe {
        core::ptr::write_bytes(global::<u32>(0x01179890), 0xFF, WORDS);
        FILL
    }
});
