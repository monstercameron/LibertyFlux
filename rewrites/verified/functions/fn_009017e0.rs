// original: 0x009017e0 input_slot_alloc (proposed)
/// Allocate one input slot object and record it in the slot table.
///
/// Calls the allocator callee for 8 bytes. On success the new object is
/// passed (as `this`, with `arg`) to the slot-initialiser callee; the
/// counter at `CNT` is incremented and the initialised pointer is stored
/// at index `counter + 1` of table `TAB_A`. When allocation fails a null
/// is stored at index `counter` of table `TAB_B` and the counter is still
/// incremented. Returns the counter value before incrementing. Cdecl, one
/// stack word.
export!(cdecl, rw_009017e0(arg: u32) -> u32 {
    unsafe {
        /// Slot counter (file VA).
        const CNT: u32 = 0x0118F4E0;
        /// Table for initialised slots, indexed by counter + 1 (file VA).
        const TAB_A: u32 = 0x0118F4EC;
        /// Table for failed allocations, indexed by counter (file VA).
        const TAB_B: u32 = 0x0118F4F0;
        const ALLOC_ID: u32 = 1;
        const INIT_ID: u32 = 2;
        let p: u32 = callee_cdecl!(ALLOC_ID, u32, 8u32);
        if p != 0 {
            let obj: u32 = callee_thiscall!(INIT_ID, u32, p, arg);
            let old = (global::<u32>(CNT)).read_unaligned();
            let new = old.wrapping_add(1);
            ((relocated(TAB_A).wrapping_add(new.wrapping_mul(4))) as *mut u32)
                .write_unaligned(obj);
            (global::<u32>(CNT)).write_unaligned(new);
            old
        } else {
            let old = (global::<u32>(CNT)).read_unaligned();
            ((relocated(TAB_B).wrapping_add(old.wrapping_mul(4))) as *mut u32)
                .write_unaligned(0);
            (global::<u32>(CNT)).write_unaligned(old.wrapping_add(1));
            old
        }
    }
});
