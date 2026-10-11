// original: 0x00BE8340 initialize_task_record_block

/// Initialize the task record block rooted at `this`. Four records start at
/// `+0x44` and are spaced `0x44` bytes apart; each receives a zero leading
/// dword, two zero qwords, a zero word at `+0x14`, and the marker `0x0100` at
/// `+0x40`. The enclosing header is cleared including byte `+0x155`, with marker
/// words `0x0100` and a dword value of 12 at its tail. The original returns
/// `this + 0x158` in EAX and uses thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00BE8340(this: u32) -> u32 {
    const RECORD_START: u32 = 0x44;
    const RECORD_STRIDE: u32 = 0x44;
    const RECORD_COUNT: u32 = 4;
    const MARKER: u16 = 0x0100;

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    unsafe {
        write_u8(this + 0x155, 0);
        for index in 0..RECORD_COUNT {
            let record = this + RECORD_START + index * RECORD_STRIDE;
            write_u32(record, 0);
            write_u32(record + 4, 0);
            write_u32(record + 8, 0);
            write_u32(record + 12, 0);
            write_u32(record + 16, 0);
            write_u16(record + 0x14, 0);
            write_u16(record + 0x40, MARKER);
        }
        write_u32(this, 0);
        write_u32(this + 4, 0);
        write_u32(this + 8, 0);
        write_u32(this + 12, 0);
        write_u32(this + 16, 0);
        write_u16(this + 0x3E, MARKER);
        write_u32(this + 0x40, 12);
        this + 0x158
    }
});
