// original: 0x008CEA30 stream_bank_limit (proposed)

/// Reads the limit dword (`+LIMIT`) of streaming bank `index`: the bank
/// pointer comes from `TABLE[index]` and must be non-null, the bank must be
/// enabled (`FLAGS[index]` non-zero), and the index `INVALID` (-90) is
/// rejected; any failure yields 0.
///
/// One stack argument (cdecl); the dword result is returned in EAX.
lf_checker_rt::export!(cdecl, rw_008CEA30(index: u32) -> u32 {
    unsafe {
        /// Base of the bank pointer table (one dword per index).
        const TABLE: u32 = 0x1173560;
        /// Base of the per-bank enable bytes.
        const FLAGS: u32 = 0x117354C;
        /// Offset of the limit dword in a bank object.
        const LIMIT: u32 = 0x3624;
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
        let enabled = (lf_checker_rt::relocated(FLAGS).wrapping_add(index) as *const u8).read();
        if enabled == 0 {
            return 0;
        }
        (bank.wrapping_add(LIMIT) as *const u32).read_unaligned()
    }
});
