// original: 0x008CE9E0 stream_bank_hold_on (proposed)

/// Sets the hold bytes of streaming bank `index`: when the index is not
/// `INVALID` (-90) and the bank is enabled (`FLAGS[index]` non-zero), writes
/// 1 to the bytes at `+HOLD0` and `+HOLD1` of the bank object from
/// `TABLE[index]`. Otherwise does nothing. (The pointer is not null-checked.)
///
/// One stack argument (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CE9E0(index: u32) -> u32 {
    unsafe {
        /// Base of the bank pointer table (one dword per index).
        const TABLE: u32 = 0x1173560;
        /// Base of the per-bank enable bytes.
        const FLAGS: u32 = 0x117354C;
        /// First hold byte offset in a bank object.
        const HOLD1: u32 = 0x35C5;
        /// Second hold byte offset in a bank object.
        const HOLD0: u32 = 0x35C4;
        /// Rejected index.
        const INVALID: u32 = 0xFFFF_FFA6;
        if index == INVALID {
            return 0;
        }
        let enabled = (lf_checker_rt::relocated(FLAGS).wrapping_add(index) as *const u8).read();
        if enabled == 0 {
            return 0;
        }
        let bank = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
            as *const u32)
            .read();
        ((bank.wrapping_add(HOLD1)) as *mut u8).write(1);
        let bank = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
            as *const u32)
            .read();
        ((bank.wrapping_add(HOLD0)) as *mut u8).write(1);
        0
    }
});
