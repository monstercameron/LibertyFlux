// original: 0x008d87a0 file_flag_set_87a0
/// Set the flag byte of entry `idx` in the global flag array.
///
/// Entries are 160 bytes apart; returns the byte offset written.
export!(cdecl, rw_008d87a0(idx: u32) -> u32 {
    unsafe {
        /// Flag array base (file VA).
        const FLAG_BASE: u32 = 0x012FB457;
        /// Entry stride in bytes (5 * 32 in the original's lea/shl).
        const STRIDE: u32 = 160;
        let off = idx.wrapping_mul(STRIDE);
        (relocated(FLAG_BASE).wrapping_add(off) as *mut u8).write(1);
        off
    }
});
