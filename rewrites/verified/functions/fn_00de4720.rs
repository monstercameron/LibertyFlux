// original: 0x00DE4720 UIGTAScrollingRow::UIGTAScrollingRow

/// Delegate the two constructor words to the base initializer, set the relocated vtable, then initialise the row state.
///
/// The zeroed 32-bit fields are at +0x1E0, +0x1E4, +0x1E8, +0x1F4, +0x1FC, +0x200, +0x204, +0x20C and +0x21C. The remaining defaults are the all-ones word at +0x1F8, bit patterns at +0x208, +0x210 and +0x214, and bytes at +0x1EC, +0x218 and +0x220. `thiscall` with two stack words; returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00de4720(this_ptr: u32, first_option: u32, second_option: u32) -> u32 {
    let _base_result: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr, first_option, second_option);
    const VTABLE_FILE_VA: u32 = 0x00EFE4BC;
    const INITIAL_TIMER_BITS: u32 = 0x4CBEBc20;
    const QUARTER_BITS: u32 = 0x3E800000;
    const INITIAL_STATE_BITS: u32 = 0x01000100;
    const EMPTY_INDEX: u32 = u32::MAX;
    const CLEARED_BYTE: u8 = 0;
    const ENABLED_BYTE: u8 = 1;
    const ZERO_WORD_FIELDS: [u32; 9] = [0x1E0, 0x1F4, 0x1FC, 0x200, 0x204, 0x1E4, 0x1E8, 0x21C, 0x20C];
    unsafe {
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
        for offset in ZERO_WORD_FIELDS {
            ((this_ptr.wrapping_add(offset)) as *mut u32).write_unaligned(0);
        }
        ((this_ptr.wrapping_add(0x1F8)) as *mut u32).write_unaligned(EMPTY_INDEX);
        ((this_ptr.wrapping_add(0x208)) as *mut u32).write_unaligned(INITIAL_TIMER_BITS);
        ((this_ptr.wrapping_add(0x210)) as *mut u32).write_unaligned(QUARTER_BITS);
        ((this_ptr.wrapping_add(0x214)) as *mut u32).write_unaligned(INITIAL_STATE_BITS);
        ((this_ptr.wrapping_add(0x1EC)) as *mut u8).write(CLEARED_BYTE);
        ((this_ptr.wrapping_add(0x218)) as *mut u8).write(ENABLED_BYTE);
        ((this_ptr.wrapping_add(0x220)) as *mut u8).write(CLEARED_BYTE);
    }
    this_ptr
});
