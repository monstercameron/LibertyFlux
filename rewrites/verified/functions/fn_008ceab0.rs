// original: 0x008CEAB0 stream_bank_slot_byte (proposed)

/// Reads one byte from a streaming bank's slot array: the bank pointer comes
/// from `TABLE[index]` and must be non-null, and the index `INVALID` (-90)
/// is rejected; either failure yields 0. Otherwise returns the byte at
/// `bank + offset + BASE`. (No enable-byte check; the offset is unchecked.)
///
/// Two stack arguments (cdecl); the byte result is returned in AL.
lf_checker_rt::export!(cdecl, rw_008CEAB0(index: u32, offset: u32) -> u32 {
    unsafe {
        /// Base of the bank pointer table (one dword per index).
        const TABLE: u32 = 0x1173560;
        /// Base offset of the slot byte array in a bank object.
        const BASE: u32 = 0x33E4;
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
        (bank.wrapping_add(offset).wrapping_add(BASE) as *const u8).read() as u32
    }
});
