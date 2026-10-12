// original: 0x005d6df0 html_link_list_free
/// Release the thread's pending link-list state. TLS slot 0 points to a
/// state record whose current cursor, end cursor, saved cursor and pending
/// count live at offsets +8, +0x10, +0x64 and +0x68. If current equals end,
/// increment the pending count; otherwise save current and advance it to
/// end. When the global cleanup object at 0x019e8668 is non-null, call slot
/// 3 of the current cursor object's vtable with the cleanup object as its
/// stack argument. Then clear the two global words at 0x019e866c and
/// 0x019e8668. A nonzero pending count is decremented and returned; otherwise
/// restore the saved cursor, clear the saved field, and return that cursor.
/// The routine takes no stack arguments and returns its result in EAX.
lf_checker_rt::export!(cdecl, rw_005d6df0() -> u32 {
    const CURRENT_CURSOR_OFFSET: u32 = 0x08;
    const END_CURSOR_OFFSET: u32 = 0x10;
    const SAVED_CURSOR_OFFSET: u32 = 0x64;
    const PENDING_COUNT_OFFSET: u32 = 0x68;
    const CLEANUP_OBJECT_ADDRESS: u32 = 0x019e_8668;
    const CLEANUP_FLAG_ADDRESS: u32 = 0x019e_866c;
    const RELEASE_SLOT_CALLEE: u32 = 1;

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

        let cleanup_object = read_u32(lf_checker_rt::relocated(CLEANUP_OBJECT_ADDRESS));
        if cleanup_object != 0 {
            let cursor_object = read_u32(state.wrapping_add(CURRENT_CURSOR_OFFSET));
            let _ = lf_checker_rt::callee_thiscall!(
                RELEASE_SLOT_CALLEE,
                u32,
                cursor_object,
                cleanup_object
            );
        }

        write_u32(lf_checker_rt::relocated(CLEANUP_FLAG_ADDRESS), 0);
        write_u32(lf_checker_rt::relocated(CLEANUP_OBJECT_ADDRESS), 0);

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
