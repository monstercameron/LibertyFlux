// original: 0x00B22D80 interior_id_tables_reset
/// Reset the process's interior lookup tables before use. The function fills
/// the 10,000-word entity lookup and the 2,000-word room-ID table with the
/// all-ones sentinel, then clears both accumulated counters. It has no
/// arguments and leaves the final all-ones value in EAX.
lf_checker_rt::export!(cdecl, rw_00b22d80() -> u32 {
    unsafe {
        const ENTITY_LOOKUP_VA: u32 = 0x0164_BA98;
        const ENTITY_LOOKUP_WORDS: usize = 0x2710;
        const ROOM_ID_TABLE_VA: u32 = 0x0165_56D8;
        const ROOM_ID_WORDS: usize = 0x7D0;
        const TOTAL_A_VA: u32 = 0x0165_7618;
        const TOTAL_B_VA: u32 = 0x0165_761C;

        core::ptr::write_bytes(
            lf_checker_rt::global::<u8>(ENTITY_LOOKUP_VA),
            0xFF,
            ENTITY_LOOKUP_WORDS * 4,
        );
        core::ptr::write_bytes(
            lf_checker_rt::global::<u8>(ROOM_ID_TABLE_VA),
            0xFF,
            ROOM_ID_WORDS * 4,
        );
        lf_checker_rt::global::<u32>(TOTAL_A_VA).write_unaligned(0);
        lf_checker_rt::global::<u32>(TOTAL_B_VA).write_unaligned(0);
        u32::MAX
    }
});
