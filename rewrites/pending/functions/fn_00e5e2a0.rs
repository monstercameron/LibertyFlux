// original: 0x00e5e2a0 net_slots_clear_e2a0
/// Clear the first two words of 59 table slots, then register the handler.
///
/// Zeroes dwords at offsets 0 and 4 of each of the 60 twelve-byte records
/// starting at `0x019D30C4`, then registers the handler at `0x00E6EE10` and
/// returns the registrar's answer.
export!(cdecl, rw_00e5e2a0() -> u32 {
    unsafe {
        /// Start of the slot table (file VA).
        const TABLE: u32 = 0x019D30C4;
        /// Handler registered for it (file VA).
        const HANDLER: u32 = 0x00E6EE10;
        /// Slots in the table; each slot is three words wide.
        const COUNT: usize = 60;
        let base = global::<u32>(TABLE);
        let mut i = 0;
        while i < COUNT {
            base.add(3 * i).write(0);
            base.add(3 * i + 1).write(0);
            i += 1;
        }
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
