// original: 0x00BE8300 clear_matching_task_records

/// Inspect four fixed-size task records beginning at `this + 0x44`. For each
/// record whose leading dword equals `search_key`, clear the leading dword,
/// the byte at `+4`, the word at `+0x14`, and the byte at `+0x40`. The records
/// are `0x44` bytes apart. The method returns the address immediately after
/// the fourth record in EAX and uses thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00BE8300(this: u32, search_key: u32) -> u32 {
    const FIRST_RECORD: u32 = 0x44;
    const RECORD_STRIDE: u32 = 0x44;
    const RECORD_COUNT: u32 = 4;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
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
        for index in 0..RECORD_COUNT {
            let record = this + FIRST_RECORD + index * RECORD_STRIDE;
            if read_u32(record) == search_key {
                write_u32(record, 0);
                write_u8(record + 4, 0);
                write_u16(record + 0x14, 0);
                write_u8(record + 0x40, 0);
            }
        }
        this + 0x154
    }
});
