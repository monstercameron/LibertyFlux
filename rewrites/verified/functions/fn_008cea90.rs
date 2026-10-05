// original: 0x008CEA90 stream_bank_hold_flag (proposed)

/// Reads the hold byte (`+HOLD`) of streaming bank `index`: the bank pointer
/// comes from `TABLE[index]` and must be non-null, and the index `INVALID`
/// (-90) is rejected; either failure yields 0. (No enable-byte check.)
///
/// One stack argument (cdecl); the byte result is returned in AL.
lf_checker_rt::export!(cdecl, rw_008CEA90(index: u32) -> u32 {
    unsafe {
        /// Base of the bank pointer table (one dword per index).
        const TABLE: u32 = 0x1173560;
        /// Offset of the hold byte in a bank object.
        const HOLD: u32 = 0x35C4;
        /// Rejected index.
        const INVALID: u32 = 0xFFFF_FFA6;
        if index == INVALID {
            return 0;
        }
        let bank = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
            as *const u32)
            .read();
        if bank == 0 {
            return 0;
        }
        (bank.wrapping_add(HOLD) as *const u8).read() as u32
    }
});
