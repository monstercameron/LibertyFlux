// original: 0x00d42610 clip_list_append (proposed)

/// Append a clip to two parallel tables with a shared count.
///
/// `this` points to a list object: dwords at `+0x00..+0x3c` are table A (clip
/// pointers), dwords at `+0x40..+0x7c` are table B (one value per clip), dword
/// at `+0x80` is the count. If the count has reached the capacity of 16 the
/// function does nothing. Otherwise it stores the `clip` pointer in table A
/// at the count index, re-reads the count, stores the dword found at
/// `clip+0x28` in table B at that index, and increments the count.
///
/// The count is re-read between the two stores exactly as the original does,
/// so an input where the first store aliases the count behaves identically.
/// The original returns nothing meaningful (eax holds the table-B value on
/// the store path and the untouched entry eax on the full path), so the
/// contract compares no return channel.
///
/// Original: thiscall, one stack word (clip pointer), callee pops 4.
lf_checker_rt::export!(thiscall, rw_00d42610(this: u32, clip: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x80;
        const TABLE_A: u32 = 0x00;
        const TABLE_B: u32 = 0x40;
        const CLIP_VALUE: u32 = 0x28;
        const CAP: u32 = 0x10;
        let n = ((this + COUNT) as *const u32).read_unaligned();
        if n >= CAP {
            return 0;
        }
        ((this + TABLE_A + n.wrapping_mul(4)) as *mut u32).write_unaligned(clip);
        let m = ((this + COUNT) as *const u32).read_unaligned();
        let v = ((clip + CLIP_VALUE) as *const u32).read_unaligned();
        ((this + TABLE_B + m.wrapping_mul(4)) as *mut u32).write_unaligned(v);
        let c = ((this + COUNT) as *const u32).read_unaligned();
        ((this + COUNT) as *mut u32).write_unaligned(c.wrapping_add(1));
        0
    }
});
