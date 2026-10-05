// original: 0x00AED2B0 kv_heap_sift_up (proposed)

/// Sift a (key, value) pair up a binary max-heap of 8-byte entries.
///
/// `base` points at the entry array (key at offset 0, payload at offset 4).
/// `index` is the slot the pair is written to if no sift happens, `root` is
/// the lowest slot the sift may reach (both compared signed). While the
/// slot is above `root` and its parent's key is below `key`, the parent
/// pair moves down one slot; the pair is then written at the hole. The
/// parent of slot `i` is `(i - 1) / 2` with the original's signed
/// divide-by-two sequence. Returns the value word (left in eax).
///
/// Original: 0x00AED2B0 (cdecl, five stack words, no calls).
lf_checker_rt::export!(cdecl, rw_00aed2b0(base: u32, index: u32, root: u32, key: u32, val: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 8;
        const KEY_OFF: u32 = 0;
        const VAL_OFF: u32 = 4;
        #[inline(always)]
        unsafe fn parent(i: i32) -> i32 {
            // `(an instruction of the original); cdq; (an instruction of the original); (an instruction of the original)`.
            unsafe {
                let t = i.wrapping_sub(1);
                let d = if t < 0 { -1i32 } else { 0i32 };
                t.wrapping_sub(d) >> 1
            }
        }
        #[inline(always)]
        unsafe fn rd(base: u32, slot: i32, off: u32) -> u32 {
            unsafe { (base.wrapping_add((slot as u32).wrapping_mul(ENTRY)).wrapping_add(off) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(base: u32, slot: i32, off: u32, v: u32) {
            unsafe { (base.wrapping_add((slot as u32).wrapping_mul(ENTRY)).wrapping_add(off) as *mut u32).write_unaligned(v) }
        }
        let root = root as i32;
        let mut slot = index as i32;
        let mut p = parent(slot);
        while slot > root {
            if rd(base, p, KEY_OFF) >= key {
                break;
            }
            wr(base, slot, KEY_OFF, rd(base, p, KEY_OFF));
            wr(base, slot, VAL_OFF, rd(base, p, VAL_OFF));
            slot = p;
            p = parent(slot);
        }
        wr(base, slot, KEY_OFF, key);
        wr(base, slot, VAL_OFF, val);
        // The original returns with eax holding the loaded value word.
        val
    }
});
