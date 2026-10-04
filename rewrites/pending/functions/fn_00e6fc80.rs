// original: 0x00E6FC80 timer_array_teardown_6a
/// Tear down the 6 timer slots of bank A and drain each slot's list.
///
/// Walks the bank from the top down; for every slot it calls the slot
/// reset hook and then repeatedly calls the entry release hook until the
/// slot reports empty. Returns the last hook answer.
export!(cdecl, rw_00e6fc80() -> u32 {
    unsafe {
        const ARRAY_TOP: u32 = 0x019F8098;
        const SLOTS: u32 = 6;
        const STRIDE: u32 = 0x10;
        let mut last: u32 = 0;
        let mut slot = relocated(ARRAY_TOP);
        for _ in 0..SLOTS {
            slot = slot.wrapping_sub(STRIDE);
            last = callee_thiscall!(1, u32, slot);
            while *((slot.wrapping_add(8)) as *const u32) != 0 {
                let arg = *(slot as *const u32);
                if arg != 0 {
                    last = callee_thiscall!(2, u32, slot, arg);
                }
            }
        }
        last
    }
});
