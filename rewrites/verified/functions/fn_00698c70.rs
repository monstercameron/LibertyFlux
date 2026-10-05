// original: 0x00698C70 bit_stream_read

/// Read a run of bits out of a packed table, advancing a cursor.
///
/// Custom convention: table holder in ECX, bit cursor pointer and bit count
/// on the stack, caller cleanup. Reads `count` bits starting at `*cursor`
/// from the word table at `[this]`, packing them low-first into the result,
/// and advances the cursor past them. Uses its own incoming count slot as
/// scratch for the table pointer (the contract switches the stack comparison
/// off) and returns with the caller's cleanup (the esp comparison is off too).
/// A zero count returns 0 without touching memory.
/// Original: 0x00698C70, 77 bytes.
lf_checker_rt::export!(thiscall, rw_00698C70(this: u32, cursor: u32, count: u32) -> u32 {
    unsafe {
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        if count == 0 {
            return 0;
        }
        let table = rd32(this);
        let mut pos = rd32(cursor);
        let mut bit: u32 = 1;
        let mut out: u32 = 0;
        let mut n = count;
        loop {
            let w = rd32(table + (pos >> 5) * 4);
            if (w >> (pos & 31)) & 1 != 0 {
                out |= bit;
            }
            pos = pos.wrapping_add(1);
            bit = bit.wrapping_add(bit);
            n -= 1;
            if n == 0 {
                break;
            }
        }
        wr32(cursor, pos);
        out
    }
});
