// original: 0x00a947d0 stream_entry_init (proposed)

/// Reset a stream entry to its empty state.
///
/// Writes zero to the words at `+0x00`, `+0x04`, `+0x08` and `+0x0c` and
/// all-ones (`-1`) to the words at `+0x10` and `+0x14`. The original stores
/// a zero byte at `+0x04` and then masks the word there with `0xff`, which
/// nets to a zero word. Returns 0. Pure leaf.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a947d0(this: u32) -> u32 {
    unsafe {
        const EMPTY_LINK: u32 = 0xffff_ffff;
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, 0);
        wr32(this.wrapping_add(4), 0);
        wr32(this.wrapping_add(8), 0);
        wr32(this.wrapping_add(0x0c), 0);
        wr32(this.wrapping_add(0x10), EMPTY_LINK);
        wr32(this.wrapping_add(0x14), EMPTY_LINK);
        0
    }
});
