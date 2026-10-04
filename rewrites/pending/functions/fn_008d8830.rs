// original: 0x008d8830 file_slot_call_8830
/// Call a fixed-this helper on table slot `idx` plus offset, flag 2.
///
/// Same slot computation as the neighbouring accessors; the second
/// forwarded argument is the constant flag 2.
export!(cdecl, rw_008d8830(a0: u32, idx: u32) -> u32 {
    unsafe {
        /// Fixed `this` pointer the original loads (file VA).
        const THIS: u32 = 0x0103E8D0;
        /// Global slot table base (file VA).
        const TABLE: u32 = 0x013053A8;
        /// Entry stride in bytes.
        const STRIDE: u32 = 100;
        /// Constant flag forwarded as the second argument.
        const FLAG: u32 = 2;
        let slot = relocated(TABLE).wrapping_add(idx.wrapping_mul(STRIDE)) as *const u32;
        let base = slot.read();
        callee_thiscall!(1, u32, relocated(THIS), base.wrapping_add(a0), FLAG)
    }
});
