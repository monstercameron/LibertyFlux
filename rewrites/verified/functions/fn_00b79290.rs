// original: 0x00b79290 task_flags_set2_clear1
/// Set flag bit 2 and clear flag bit 1 of the word at offset `0xc`.
///
/// Reads the flags word, forces bit 2 on and bit 1 off, writes it back,
/// and returns the new value.
export!(thiscall, rw_00b79290(this: u32) -> u32 {
    unsafe {
        let p = (this + 0x0c) as *mut u32;
        let v = (*p & !2) | 4;
        *p = v;
        v
    }
});
