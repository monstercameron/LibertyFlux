// original: 0x00a9f430 stream_acquire_slot (proposed)

/// Take a slot from the free list and append it to the live list.
///
/// `this` points to the table object. The head of the free list at
/// `+0x1c00` is popped; a null head means the table is exhausted and the
/// result is null. Otherwise the popped slot is appended to the live list
/// at `+0x1c0c` and returned.
///
/// Original: 0x00a9f430 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00a9f430(this: u32) -> u32 {
    unsafe {
        const FREE_OFF: u32 = 0x1c00;
        const LIVE_OFF: u32 = 0x1c0c;
        const POP: u32 = 1;
        const APPEND: u32 = 2;
        let slot: u32 =
            lf_checker_rt::callee_thiscall!(POP, u32, this.wrapping_add(FREE_OFF));
        if slot == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(APPEND, u32, this.wrapping_add(LIVE_OFF), slot);
        slot
    }
});
