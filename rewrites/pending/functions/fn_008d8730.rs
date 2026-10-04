// original: 0x008d8730 file_slot_call_8730
/// Call a fixed-this helper on table slot `idx` plus a byte offset.
///
/// The original scales the index by the 100-byte entry stride, loads the
/// entry's base pointer from the global slot table, adds the caller's
/// offset, and forwards the sum as the single argument.
export!(cdecl, rw_008d8730(a0: u32, idx: u32) -> u32 {
    unsafe {
        /// Fixed `this` pointer the original loads (file VA).
        const THIS: u32 = 0x0103E8D0;
        /// Global slot table base (file VA).
        const TABLE: u32 = 0x013053A8;
        /// Entry stride in bytes.
        const STRIDE: u32 = 100;
        let slot = relocated(TABLE).wrapping_add(idx.wrapping_mul(STRIDE)) as *const u32;
        let base = slot.read();
        callee_thiscall!(1, u32, relocated(THIS), base.wrapping_add(a0))
    }
});
