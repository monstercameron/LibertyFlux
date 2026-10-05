// original: 0x008d6ee0 build_grid_block_and_test
/// Grid-block builder: allocates a fixed 0x1828-byte block on the stack,
/// fills 64 records of 0x60 bytes from three global floats plus constants,
/// then runs a dispatch call and a test call over frame slots.
///
/// Original: cdecl/4 (two forwarded words, a compared-then-discarded word,
/// a signed gate). The allocator runs NATIVELY (fixed size, no count to
/// pin). The 64 records are write-only as far as any consumer goes, so the
/// proof snapshots the head 64 words (pattern) and the tail 58 words
/// (completion); the middle holds by uniformity. One dispatch argument
/// re-reads the a2 frame slot (still a2: nothing overwrote it).
/// Exit EAX is the scripted cookie answer.
export!(cdecl, rw_008d6ee0(a0: u32, a1: u32, a2v: u32, a3: u32) -> u32 {
    unsafe {
        let a_val = if (a3 as i32) < 0 {
            callee_cdecl!(1, u32,)
        } else {
            a3
        };
        let f3 = *global::<u32>(0x1B4B320);
        let f2 = *global::<u32>(0x1B4B324);
        let f1 = *global::<u32>(0x1B4B328);
        // Native-alloca model: the full 0x1828 block, zeroed (the
        // contract zeroes uninitialized stack, so gaps match too).
        let mut buf = [0u32; 1546];
        for k in 0..64u32 {
            let b = (6 + k * 24) as usize;
            buf[b + 4] = f3;
            buf[b + 5] = f2;
            buf[b + 6] = f1;
            buf[b + 8] = f3;
            buf[b + 9] = f2;
            buf[b + 10] = f1;
            buf[b + 12] = f3;
            buf[b + 13] = f2;
            buf[b + 14] = f1;
            buf[b + 19] = 0xFFFF;
        }
        let buf_ptr = buf.as_ptr() as u32;
        // arg2 is the a2 slot's value (a2 itself: nothing overwrote it);
        // arg3 points at the record block base.
        let _: u32 = callee_cdecl!(
            2, u32, a0, a1,
            a2v, buf_ptr + 0x18,
            a_val, 0x40, 4
        );
        // Test call anchors: the record block base (tail snapshot base)
        // and the a2 slot address (both skipped; the slot's value is
        // compared-then-discarded on the original side, unobserved).
        let _: u32 = callee_cdecl!(3, u32, buf_ptr + 0x18, buf_ptr + 0xC);
        callee_cdecl!(4, u32,)
    }
});
