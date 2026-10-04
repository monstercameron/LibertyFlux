// original: 0x0089E890 aud_seq_advance_fire
// ---------------------------------------------------------------------------
// 0x0089E890: advance this sequential sound's cursor and fire the newly
// selected entry. Does nothing (returning 0) unless the sound is armed
// (+0xD2) and the next cursor still falls inside the entry count (+0xC8).
// (Entry EAX pinned to 0: the idle paths return it untouched.)
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089E890(this_ptr: u32, arg: u32) -> u32 {
    const BANK_ROW: u32 = 0x6F40;
    const BANK_BASE_OFF: u32 = 0x6F10;
    let armed = unsafe { *((this_ptr.wrapping_add(0xD2)) as *const u8) };
    if armed == 0 {
        return 0;
    }
    let cursor = unsafe { *((this_ptr.wrapping_add(0xCC)) as *const u32) }.wrapping_add(1);
    let count = unsafe { *((this_ptr.wrapping_add(0xC8)) as *const u32) };
    if (cursor as i32) <= 0 || (cursor as i32) >= (count as i32) {
        return 0;
    }
    let bank_idx = unsafe { *((this_ptr.wrapping_add(0x40)) as *const u8) } as u32;
    let sel = unsafe { *((this_ptr.wrapping_add(0xD1)) as *const u8) } as u32;
    let bank_table = unsafe { *global::<u32>(0x115D988) };
    let stride = unsafe { *global::<u32>(0x115D964) };
    let row = unsafe {
        *((bank_table
            .wrapping_add(bank_idx.wrapping_mul(BANK_ROW))
            .wrapping_add(BANK_BASE_OFF)) as *const u32)
    };
    let table = row.wrapping_add(stride.wrapping_mul(sel));
    let entry = unsafe { *((table.wrapping_add(cursor.wrapping_mul(4))) as *const u32) };
    let buf = this_ptr.wrapping_add(0xB0);
    let answer = callee_thiscall!(1, u32, this_ptr, entry, arg, buf);
    if arg < 2 {
        unsafe { *((this_ptr.wrapping_add(arg.wrapping_mul(4)).wrapping_add(0x9C)) as *mut u32) = 0 };
    }
    answer
});
