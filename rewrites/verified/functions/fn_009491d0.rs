// original: 0x009491d0 handle_table_reset_second10
/// Reset a further ten handle slots to empty (0xFFFFFFFF).
export!(cdecl, rw_009491d0() -> u32 {
    unsafe {
        for i in 0..10u32 {
            *global::<u32>(0x011D9578 + i * 4) = 0xFFFF_FFFF;
        }
        0
    }
});
