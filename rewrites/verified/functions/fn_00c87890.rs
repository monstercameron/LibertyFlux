// original: 0x00c87890 audio_entry_find (proposed)
///
/// Searches the entry table rooted at `this` for `key`. The half-word at
/// `this+2` is the starting entry index (unsigned; 0 means empty, return
/// null). Entry `i` sits at `this - 0x30 + i * 0x60`; its key is the
/// double-word at +0x44 and the next index is the half-word at +0x58
/// (0 ends the chain). Returns the matching entry's address, or null.
/// Thiscall, one stack argument.

lf_checker_rt::export!(thiscall, rw_00c87890(this: u32, key: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
    #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        const ROW: u32 = 0x60;
        const BASE_OFF: u32 = 0x30;
        const KEY_OFF: u32 = 0x44;
        const NEXT_OFF: u32 = 0x58;
        let mut idx = rd16(this.wrapping_add(2)) as u32;
        if idx == 0 {
            return 0;
        }
        loop {
            let entry = this.wrapping_sub(BASE_OFF).wrapping_add(idx.wrapping_mul(ROW));
            if rd32(entry.wrapping_add(KEY_OFF)) == key {
                return entry;
            }
            idx = rd16(entry.wrapping_add(NEXT_OFF)) as u32;
            if idx == 0 {
                return 0;
            }
        }
    }
});
