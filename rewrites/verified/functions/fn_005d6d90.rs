// original: 0x005d6d90 html_link_list_append
/// Reserve a slot for the supplied link item from the current thread's
/// list state, write the item through the slot returned by the list helper,
/// and return the updated pending count or restored cursor. TLS slot 0
/// points to the state record: offset +8 is its current cursor, +0x10 its
/// end cursor, +0x64 a saved cursor, and +0x68 the pending count. When the
/// current and end cursors match, increment the pending count. Otherwise
/// save the current cursor and advance to the end. The helper is called with
/// the item as both this and its one stack argument; thiscall cleans that
/// argument. After the slot write, a nonzero pending count is decremented
/// and returned. If it is zero, restore the saved cursor, clear the saved
/// field, and return that cursor in EAX.
lf_checker_rt::export!(thiscall, rw_005d6d90(this: u32) -> u32 {
    const CURRENT_CURSOR_OFFSET: u32 = 0x08;
    const END_CURSOR_OFFSET: u32 = 0x10;
    const SAVED_CURSOR_OFFSET: u32 = 0x64;
    const PENDING_COUNT_OFFSET: u32 = 0x68;
    const APPEND_SLOT_CALLEE: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let state = lf_checker_rt::tls_slot(0);
        let current_cursor = read_u32(state.wrapping_add(CURRENT_CURSOR_OFFSET));
        let end_cursor = read_u32(state.wrapping_add(END_CURSOR_OFFSET));

        if current_cursor == end_cursor {
            let pending = read_u32(state.wrapping_add(PENDING_COUNT_OFFSET));
            write_u32(state.wrapping_add(PENDING_COUNT_OFFSET), pending.wrapping_add(1));
        } else {
            write_u32(state.wrapping_add(SAVED_CURSOR_OFFSET), current_cursor);
            write_u32(state.wrapping_add(CURRENT_CURSOR_OFFSET), end_cursor);
        }

        let slot = lf_checker_rt::callee_thiscall!(APPEND_SLOT_CALLEE, u32, this, this);
        write_u32(slot, this);

        let pending = read_u32(state.wrapping_add(PENDING_COUNT_OFFSET));
        if pending != 0 {
            let remaining = pending.wrapping_sub(1);
            write_u32(state.wrapping_add(PENDING_COUNT_OFFSET), remaining);
            remaining
        } else {
            let saved_cursor = read_u32(state.wrapping_add(SAVED_CURSOR_OFFSET));
            write_u32(state.wrapping_add(CURRENT_CURSOR_OFFSET), saved_cursor);
            write_u32(state.wrapping_add(SAVED_CURSOR_OFFSET), 0);
            saved_cursor
        }
    }
});
