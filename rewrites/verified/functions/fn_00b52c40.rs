// original: 0x00B52C40 euphoria_write_slot_frames

//! Copy two four-word frame records into slots derived from the current count.
//!
//! `this` stores the count at byte offset `0x0c`. The first input record is
//! copied to slot `count + 3`; the second is copied to slot `count + 0x23`.
//! Each slot is 16 bytes. The function then marks byte `this[0x10 + count]`
//! and the state byte at offset `0x04`, and returns the original count in
//! EAX. Record words are copied unchanged, including floating-point bit
//! patterns in the middle two words.
lf_checker_rt::export!(thiscall, rw_b52c40(this: *mut u8, first_frame: *const u32, second_frame: *const u32) -> u32 {
    unsafe {
        const COUNT_OFFSET: usize = 0x0c;
        const FIRST_SLOT: u32 = 3;
        const SECOND_SLOT: u32 = 0x23;
        const SLOT_STRIDE: usize = 0x10;
        let count = (this.add(COUNT_OFFSET) as *const u32).read_unaligned();
        let first_dst = this.add(count.wrapping_add(FIRST_SLOT) as usize * SLOT_STRIDE);
        let second_dst = this.add(count.wrapping_add(SECOND_SLOT) as usize * SLOT_STRIDE);
        for word in 0..4 {
            let first_value = first_frame.add(word).read_unaligned();
            let second_value = second_frame.add(word).read_unaligned();
            (first_dst.add(word * 4) as *mut u32).write_unaligned(first_value);
            (second_dst.add(word * 4) as *mut u32).write_unaligned(second_value);
        }
        this.add(0x10 + count as usize).write(1);
        this.add(0x04).write(1);
        count
    }
});
