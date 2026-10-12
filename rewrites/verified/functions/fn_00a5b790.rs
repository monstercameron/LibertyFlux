// original: 0x00A5B790 copy_record16

/// Copies four 32-bit words from the source record to the destination in
/// program order. The first three source words are captured before writing;
/// the fourth is loaded after those writes, preserving behavior for overlapping
/// regions. Arguments are cdecl source and destination pointers. The EAX value
/// left by the original is not a defined result; the observable effect is the
/// 16-byte destination update.
lf_checker_rt::export!(cdecl, rw_00A5B790(_ignored: u32, source_record: u32, destination: u32) -> u32 {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let first_word = unsafe { read_u32(source_record) };
    let second_word = unsafe { read_u32(source_record.wrapping_add(4)) };
    let third_word = unsafe { read_u32(source_record.wrapping_add(8)) };
    unsafe {
        write_u32(destination, first_word);
        write_u32(destination.wrapping_add(4), second_word);
        write_u32(destination.wrapping_add(8), third_word);
    }
    let fourth_word = unsafe { read_u32(source_record.wrapping_add(12)) };
    unsafe { write_u32(destination.wrapping_add(12), fourth_word) };
    0
});
