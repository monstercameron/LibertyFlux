// original: 0x00A7D750 timing_record_init
/// Initialize a timing record in place. Clear the low three flag bits in the word at +0x08, set the 16-bit field at +0x0c and byte at +0x0e to all ones, clear the first two dwords, and return the same record pointer.
lf_checker_rt::export!(thiscall, rw_00a7d750(this: u32) -> u32 {
    const FLAGS_OFFSET: u32 = 0x08;
    const LOW_FLAG_MASK: u32 = 0x07;
    const SHORT_OFFSET: u32 = 0x0C;
    const BYTE_OFFSET: u32 = 0x0E;
    unsafe {
        let flags = (this.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned();
        (this.wrapping_add(FLAGS_OFFSET) as *mut u32).write_unaligned(flags & !LOW_FLAG_MASK);
        (this.wrapping_add(SHORT_OFFSET) as *mut u16).write_unaligned(u16::MAX);
        (this.wrapping_add(BYTE_OFFSET) as *mut u8).write(u8::MAX);
        (this as *mut u32).write_unaligned(0);
        (this.wrapping_add(4) as *mut u32).write_unaligned(0);
    }
    this
});
