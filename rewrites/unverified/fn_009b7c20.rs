// original: 0x009B7C20 input_find_named_entry_flag (proposed)

/// Searches the entry table for the id and reports its flag. Reads the entry count and the entry-pointer array from their globals, scans while the signed index stays below the count, and when an entry's id at +0x53c matches returns whether its flag byte at +0x558 has bit 0 set; returns 0 when nothing matches. The original's null-entry check past the match is dead (a null entry faults at the id load first) and is kept as written.
///
/// Original: 0x009B7C20 (stdcall).
lf_checker_rt::export!(stdcall, rw_009B7C20(target: u32) -> u32 {
    const COUNT_GLOBAL_VA: u32 = 0x0118_D7F4;
    const ARRAY_GLOBAL_VA: u32 = 0x0118_D7F0;
    const ID_OFFSET: u32 = 0x53C;
    const FLAG_OFFSET: u32 = 0x558;

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    unsafe {
        let count = (lf_checker_rt::global::<u32>(COUNT_GLOBAL_VA).read_unaligned() & 0xFFFF) as i32;
        if count <= 0 {
            return 0;
        }
        let array = lf_checker_rt::global::<u32>(ARRAY_GLOBAL_VA).read_unaligned();
        let mut index: i32 = 0;
        while index < count {
            let entry = read_word(array + (index as u32) * 4);
            if read_word(entry + ID_OFFSET) == target {
                if entry == 0 {
                    return 0;
                }
                if read_byte(entry + FLAG_OFFSET) & 1 != 0 {
                    return 1;
                }
                return 0;
            }
            index += 1;
        }
        0
    }
});
