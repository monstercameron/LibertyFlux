// original: 0x008CEAE0 stream_bank_slot_addr (proposed)

/// Address of a slot in a streaming bank's slot array: the bank pointer comes
/// from `TABLE[index]` and must be non-null, the bank must be enabled
/// (`FLAGS[index]` non-zero), and the index `INVALID` (-90) is rejected; any
/// failure yields 0. Otherwise returns `bank + (a * 50 + b) * 120 + 4`, all
/// in wrapping arithmetic.
///
/// Three stack arguments (cdecl); the address is returned in EAX. The result
/// is only computed, never dereferenced.
lf_checker_rt::export!(cdecl, rw_008CEAE0(index: u32, a: u32, b: u32) -> u32 {
    unsafe {
        /// Base of the bank pointer table (one dword per index).
        const TABLE: u32 = 0x1173560;
        /// Base of the per-bank enable bytes.
        const FLAGS: u32 = 0x117354C;
        /// Outer multiplier of the slot number.
        const OUTER: u32 = 50;
        /// Slot stride in bytes.
        const STRIDE: u32 = 120;
        /// Header skipped before the first slot.
        const HEADER: u32 = 4;
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
        let slot = a.wrapping_mul(OUTER).wrapping_add(b);
        bank.wrapping_add(slot.wrapping_mul(STRIDE)).wrapping_add(HEADER)
    }
});
