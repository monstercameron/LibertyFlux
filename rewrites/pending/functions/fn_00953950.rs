// original: 0x00953950 advance_slot_cursor
/// Append a terminator at the cursor and advance the slot ring.
///
/// Writes a zero byte at the cursor, bumps the cursor, and always marks the
/// current slot active. When the ring is already active it also rotates: the
/// next slot index is the cursor count modulo the ring size, the cursor
/// resets, and the new slot's buffer is selected and cleared. Returns 1
/// after a rotation, 0 otherwise.
export!(cdecl, rw_00953950() -> u32 {
    unsafe {
        let cursor = global::<u32>(0x11FA008);
        let buf = global::<u32>(0x11FA00C);
        *((*buf).wrapping_add(*cursor) as *mut u8) = 0;
        let cur = *global::<u8>(0x11FA010);
        *cursor = (*cursor).wrapping_add(1);
        *(relocated(0x11F7018).wrapping_add(cur as u32) as *mut u8) = 1;
        if *global::<u8>(0x11F6FFA) == 1 {
            return 0;
        }
        let size = *global::<u8>(0x11F6FFD) as u32;
        let next = ((cur as u32 + 1) % size) as u8;
        *cursor = 0;
        *global::<u8>(0x11FA010) = next;
        *(relocated(0x11F7018).wrapping_add(next as u32) as *mut u8) = 2;
        let selected = *(relocated(0x11F7000).wrapping_add(next as u32 * 4) as *const u32);
        *buf = selected;
        *(selected as *mut u8) = 0;
        (selected & 0xFFFFFF00) | 1
    }
});
