// original: 0x00bbbb20 NativeImpl_TASK_EXTEND_ROUTE
/// Append a waypoint to the patrol-route table unless it is full.
///
/// Reads the route-entry count from its global slot. When the count is below
/// eight (signed comparison, so a negative count also stores), stores the
/// three argument words plus a zero fourth word into the count-th 16-byte row
/// of the route table and increments the count; when the table is full nothing
/// is written. Returns the row pointer when a row was
/// written, otherwise the count on entry (the original reuses EAX for the
/// slot address, so the two paths return different things).
/// The original reads its fourth stored word from an uninitialized stack slot;
/// the checker fills such slots with the contract's `stack_fill` (zero here),
/// so the rewrite stores zero.
export!(cdecl, rw_00bbbb20(f0: u32, f1: u32, f2: u32) -> u32 {
    unsafe {
        let count = global::<u32>(0x171BE70);
        let n = *count;
        if (n as i32) < 8 {
            let slot = (relocated(0x171BE80).wrapping_add(n.wrapping_shl(4))) as *mut u32;
            *slot.add(0) = f0;
            *slot.add(1) = f1;
            *slot.add(2) = f2;
            *slot.add(3) = 0;
            // The increment re-reads the counter slot after the stores: when
            // the count is -1 the slot aliases the counter and the stored
            // first word is what gets incremented.
            let c = *count;
            *count = c.wrapping_add(1);
            slot as u32
        } else {
            n
        }
    }
});
