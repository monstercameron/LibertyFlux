// original: 0x0088ac00 audio_entity_event_append
/// Appends one event record to an audio entity's table row.
///
/// `key` selects the row (`key * 0x6F40` bytes past the table base at
/// `this+0xE8`). Does nothing once the row's record count (byte at row
/// offset `0x6F0C`) reaches 12. Otherwise writes the two flag bytes, the
/// parameter word, copies 6 bytes from `src6` and 24 bytes from `src24`
/// into the new record, and bumps the count.
export!(thiscall, rw_0088ac00(
    this_ptr: *const u8,
    key: u32,
    flag0: u32,
    param: u32,
    src6: *const u8,
    flag1: u32,
    src24: *const u8,
) -> u32 {
    unsafe {
        let table = *(this_ptr.add(0xE8) as *const u32) as *const u8;
        let row = table.add(key.wrapping_mul(0x6F40) as usize) as *mut u8;
        let count = *row.add(0x6F0C);
        if count >= 12 {
            return 0;
        }
        let slot = (count as u32).wrapping_mul(40);
        *row.add(slot.wrapping_add(0x6CE4) as usize) = flag0 as u8;
        *row.add(slot.wrapping_add(0x6CE5) as usize) = flag1 as u8;
        core::ptr::write_unaligned(
            row.add(slot.wrapping_add(0x6CE0) as usize) as *mut u32,
            param,
        );
        let dst6 = row.add(slot.wrapping_add(0x6CC0) as usize) as *mut u8;
        core::ptr::copy_nonoverlapping(src6, dst6, 6);
        let dst24 = row.add(slot.wrapping_add(0x6CC8) as usize) as *mut u8;
        core::ptr::copy_nonoverlapping(src24, dst24, 24);
        *row.add(0x6F0C) = count.wrapping_add(1);
        0
    }
});
