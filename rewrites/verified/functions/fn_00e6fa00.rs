// original: 0x00E6FA00 timer_array_teardown_16
/// Tear down the 16 timer slots: reset each, then drain its two lists.
///
/// Walks the slot array from the top down; for every slot it calls the
/// slot reset hook and then repeatedly calls the per-entry release hooks
/// until each list reports empty. Returns the last hook answer.
export!(cdecl, rw_00e6fa00() -> u32 {
    unsafe {
        const ARRAY_TOP: u32 = 0x019F386C;
        const SLOTS: u32 = 16;
        const STRIDE: u32 = 0x1C;
        let mut last: u32 = 0;
        let mut slot = relocated(ARRAY_TOP);
        for _ in 0..SLOTS {
            slot = slot.wrapping_sub(STRIDE);
            last = callee_thiscall!(1, u32, slot.wrapping_sub(0x0C));
            while *((slot.wrapping_add(0x0C)) as *const u32) != 0 {
                let arg = *((slot.wrapping_add(0x04)) as *const u32);
                if arg != 0 {
                    last = callee_thiscall!(2, u32, slot.wrapping_add(0x04), arg);
                }
            }
            while *(slot as *const u32) != 0 {
                let arg = *((slot.wrapping_sub(8)) as *const u32);
                if arg != 0 {
                    last = callee_thiscall!(3, u32, slot.wrapping_sub(8), arg);
                }
            }
        }
        last
    }
});
