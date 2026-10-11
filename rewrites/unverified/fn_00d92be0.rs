// original: 0x00D92BE0 mainloop_timing_set_record_active_flags

/// Set or clear the active marker on every timing record owned by the object.
///
/// `timing_object` contains a record-array pointer at byte offset `0x6c` and
/// an unsigned record count at `0x7c`. Records are `0x28` bytes apart, and
/// their first word carries the active marker in bit `0x2000`. The one stack
/// argument is interpreted as a byte: zero clears that bit and any nonzero
/// low byte sets it. The function has thiscall cleanup and returns no semantic
/// value.
lf_checker_rt::export!(thiscall, rw_00d92be0(timing_object: u32, enabled: u32) -> u32 {
    unsafe {
        const TIMING_RECORD_ARRAY: u32 = 0x6c;
        const TIMING_RECORD_COUNT: u32 = 0x7c;
        const TIMING_RECORD_STRIDE: u32 = 0x28;
        const RECORD_ACTIVE_FLAG: u32 = 0x2000;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let records = read_u32(timing_object.wrapping_add(TIMING_RECORD_ARRAY));
        let count = read_u32(timing_object.wrapping_add(TIMING_RECORD_COUNT));
        let mut index = 0u32;
        while index < count {
            let flag_address = records.wrapping_add(index.wrapping_mul(TIMING_RECORD_STRIDE));
            let old_flags = read_u32(flag_address);
            let new_flags = if enabled as u8 != 0 {
                old_flags | RECORD_ACTIVE_FLAG
            } else {
                old_flags & !RECORD_ACTIVE_FLAG
            };
            write_u32(flag_address, new_flags);
            index = index.wrapping_add(1);
        }
    }
    0
});
