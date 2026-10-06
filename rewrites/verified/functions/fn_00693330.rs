// original: 0x00693330 track_array_insert (proposed)

/// Inserts `new_elem` into a sorted pointer array at the cursor `cursor`
/// (a pointer to the slot, not the base): while the new track's key is
/// strictly below the previous slot's key (both keys unsigned 24-bit
/// values, byte at +5 shifted left 16 or-ed with word at +6, compared with
/// an unsigned below), the previous slot moves up one and the cursor steps
/// down; then the new element is stored at the cursor. No return value.
///
/// Original: 0x00693330 (ECX plus one stack argument with caller cleanup,
/// which has no Rust equivalent, so the esp check is off for this function).
lf_checker_rt::export!(thiscall, rw_00693330(cursor: u32, new_elem: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn track_key(t: u32) -> u32 {
            unsafe { ((rd8(t.wrapping_add(5)) as u32) << 16) | (rd16(t.wrapping_add(6)) as u32) }
        }

        let want = track_key(new_elem);
        let mut c = cursor;
        while want < track_key(rd32(c.wrapping_sub(4))) {
            wr32(c, rd32(c.wrapping_sub(4)));
            c = c.wrapping_sub(4);
        }
        wr32(c, new_elem);
        0
    }
});
