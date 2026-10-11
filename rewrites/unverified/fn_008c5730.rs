// original: 0x008C5730 file_memory_state_initialize
/// Initialize the file-memory state object: clear its fixed fields and
/// fourteen paired arrays, construct each child slot through the checker
/// stand-in, set the invalid-index sentinel, and clear the shared UTF-16 table.
lf_checker_rt::export!(thiscall, rw_008c5730(this: u32) -> u32 {
    const CHILD_CONSTRUCTOR: u32 = 1;
    const UTF16_SCRATCH_TABLE_FILE_VA: u32 = 0x116B_F68;
    const DESCRIPTOR_COUNT: u32 = 14;
    unsafe {
        for offset in [0, 4, 8, 12] {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        for index in 0..DESCRIPTOR_COUNT {
            ((this + 0x10 + index * 8) as *mut u32).write_unaligned(0);
            ((this + 0x14 + index * 8) as *mut u32).write_unaligned(0);
            ((this + 0x80 + index * 8) as *mut u32).write_unaligned(0);
            ((this + 0x84 + index * 8) as *mut u32).write_unaligned(0);
        }
        for index in 0..DESCRIPTOR_COUNT {
            let child = this + 0x10C + index * 0x10;
            let _ = lf_checker_rt::callee_thiscall!(CHILD_CONSTRUCTOR, u32, child);
        }
        ((this + 0x264) as *mut u32).write_unaligned(0);
        ((this + 0x268) as *mut u32).write_unaligned(0);
        ((this + 0x263) as *mut u8).write(0);
        ((this + 0x260) as *mut u16).write_unaligned(0x65);
        ((this + 0x25C) as *mut u32).write_unaligned(u32::MAX);
        for index in 0..DESCRIPTOR_COUNT {
            ((this + 0xF0 + index) as *mut u8).write(0);
            ((this + 0xFE + index) as *mut u8).write(0);
            ((this + 0x1EC + index * 8) as *mut u8).write(0);
        }
        let table = lf_checker_rt::relocated(UTF16_SCRATCH_TABLE_FILE_VA);
        for index in 0..64 {
            (table.wrapping_add(index * 2) as *mut u16).write_unaligned(0);
        }
        this
    }
});
