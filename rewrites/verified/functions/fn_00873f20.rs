// original: 0x00873f20 rage::crmtManagerMixer::vf7
// Delete one entry and shift the tail of the linked table down over it,
// then clear the freed record at the end. (thiscall/1)
export!(thiscall, rw_00873f20(this_ptr: u32, index: u32) -> () {
    unsafe {
        const CAPACITY: u32 = 0x20;
        const TAIL_WORD: usize = 0x118 / 4;
        let mid = (this_ptr as *const u32).add(2).read();
        if mid == 0 {
            return;
        }
        let table = (mid as *const u32).add(2).read();
        if table == 0 {
            return;
        }
        let next = index.wrapping_add(1);
        if next < CAPACITY {
            let count = CAPACITY.wrapping_sub(next).wrapping_mul(2) & 0x3FFF_FFFE;
            let dst_word = next.wrapping_add(3).wrapping_mul(2) as usize;
            let src_word = next.wrapping_add(4).wrapping_mul(2) as usize;
            let words = table as *mut u32;
            core::ptr::copy(
                words.add(src_word),
                words.add(dst_word),
                count as usize,
            );
        }
        let words = table as *mut u32;
        words.add(TAIL_WORD).write(0);
        words.add(TAIL_WORD + 1).write(0);
    }
});
