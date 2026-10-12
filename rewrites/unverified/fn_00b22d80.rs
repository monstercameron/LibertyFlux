// original: 0x00b22d80 interior_id_tables_reset
//! Reset the interior entity-lookup and room-ID tables to their empty state.
//!
//! The function takes no arguments and calls nothing. It fills the large
//! entity-lookup table (ten thousand words) and the smaller room-ID table
//! (two thousand words) with the all-ones "no entry" marker, then clears the
//! two running totals that count accumulated entries to zero. The return
//! register carries no value the caller uses, so the proof does not compare it.
#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe {
        (address as *mut u32).write_unaligned(value);
    }
}

lf_checker_rt::export!(cdecl, rw_00b22d80() -> u32 {
    unsafe {
        const ENTITY_LOOKUP: u32 = 0x0164_BA98;
        const ENTITY_LOOKUP_WORDS: usize = 0x2710;
        const ROOM_ID_TABLE: u32 = 0x0165_56D8;
        const ROOM_ID_WORDS: usize = 0x7D0;
        const TOTAL_A: u32 = 0x0165_7618;
        const TOTAL_B: u32 = 0x0165_761C;
        core::ptr::write_bytes(
            lf_checker_rt::relocated(ENTITY_LOOKUP) as *mut u8,
            0xFF,
            ENTITY_LOOKUP_WORDS * 4,
        );
        core::ptr::write_bytes(
            lf_checker_rt::relocated(ROOM_ID_TABLE) as *mut u8,
            0xFF,
            ROOM_ID_WORDS * 4,
        );
        write_u32(lf_checker_rt::relocated(TOTAL_A) as u32, 0);
        write_u32(lf_checker_rt::relocated(TOTAL_B) as u32, 0);
        0
    }
});
