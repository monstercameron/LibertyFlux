// original: 0x00D9EF40 copy_stream_header
/// Copies the byte and word at object offsets 0x30 and 0x34, then asks the base copy helper to transfer six words from offset 0x38 into destination offset 8; returns the destination pointer.
lf_checker_rt::export!(thiscall, rw_s675_00d9ef40(this: u32, destination: u32) -> u32 {
    const FLAG_OFFSET: u32 = 0x30;
    const WORD_OFFSET: u32 = 0x34;
    const SOURCE_OFFSET: u32 = 0x38;
    let flag = unsafe { (this.wrapping_add(FLAG_OFFSET) as *const u8).read() };
    let word = unsafe { (this.wrapping_add(WORD_OFFSET) as *const u32).read_unaligned() };
    unsafe {
        (destination as *mut u8).write(flag);
        (destination.wrapping_add(4) as *mut u32).write_unaligned(word);
    }
    let output_tail = destination.wrapping_add(8);
    let source_tail = this.wrapping_add(SOURCE_OFFSET);
    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, output_tail, source_tail);
    destination
});
