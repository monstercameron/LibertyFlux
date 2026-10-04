// original: 0x00949120 handle_tables_reset10
/// Reset ten handle slots to empty (0xFFFFFFFF) and clear the ten matching
/// per-slot flag bytes.
export!(cdecl, rw_00949120() -> u32 {
    unsafe {
        for i in 0..10u32 {
            *global::<u32>(0x011D9550 + i * 4) = 0xFFFF_FFFF;
            *global::<u8>(0x011E66D0 + i * 0x18) = 0;
        }
        0
    }
});
