// original: 0x0091EEE0 wstr_cat
/// Append one wide string to another and return the destination.
///
/// Scans `dst` for its NUL terminator, copies `src` over it including its
/// own terminator, and returns `dst`. Either string may be empty. (The batch
/// list and the inventory both give this function's size as 69 bytes, which
/// ends mid-epilogue; the true size through `ret` is 76.)
export!(cdecl, rw_0091eee0(dst: u32, src: u32) -> u32 {
    unsafe {
        let d = dst as *mut u16;
        let s = src as *const u16;
        let mut e = d;
        while *e != 0 {
            e = e.add(1);
        }
        let mut i: usize = 0;
        while *s.add(i) != 0 {
            *e = *s.add(i);
            e = e.add(1);
            i += 1;
        }
        *e = 0;
        dst
    }
});
