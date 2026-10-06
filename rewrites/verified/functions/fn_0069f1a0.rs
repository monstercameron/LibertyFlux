// original: 0x0069F1A0 fill_byte_dword_pairs
/// Stamp a tag byte and fill a 512-byte table with (byte, dword) pairs.
///
/// Writes `tag` (low byte of `a0`) to `this+6` and `this+7` first. When the
/// buffer pointer at `this+12` is null it stops there; otherwise it writes
/// 64 entries of 8 bytes at `buf[8*k] = tag` (one byte) and `buf[8*k+4] =
/// a1`. The pointer is re-read from `this+12` before every store, exactly as
/// the original does, so an aliasing buffer that overwrites the pointer
/// mid-loop behaves the same. Returns nothing meaningful (eax is the loop
/// counter 0x200 on the fill path and untouched entry garbage on the
/// null path), so the contract compares no return value.
/// Original: thiscall, two stack words, callee cleanup 8, no calls.
lf_checker_rt::export!(thiscall, rw_0069f1a0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const TAG0: u32 = 6;
        const TAG1: u32 = 7;
        const BUF: u32 = 12;
        const END: u32 = 0x200;
        const STEP: u32 = 8;
        let tag = a0 as u8;
        let obj = this as *mut u8;
        obj.add(TAG0 as usize).write(tag);
        obj.add(TAG1 as usize).write(tag);
        if (this.wrapping_add(BUF) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        let mut off = 0u32;
        loop {
            let buf = (this.wrapping_add(BUF) as *const u32).read_unaligned();
            off += STEP;
            (buf.wrapping_add(off).wrapping_sub(STEP) as *mut u8).write(tag);
            let buf2 = (this.wrapping_add(BUF) as *const u32).read_unaligned();
            (buf2.wrapping_add(off).wrapping_sub(4) as *mut u32).write_unaligned(a1);
            if off >= END {
                break;
            }
        }
        0
    }
});