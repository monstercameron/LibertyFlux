// original: 0x00a297a0 task_record_replace_reference_and_flags

/// Release the old task reference when non-null, store and retain the new
/// reference, copy one input byte to offset 0x210, and replace only bit zero
/// of the flag byte at 0x216 with the corresponding input bit.
lf_checker_rt::export!(thiscall, rw_00a297a0(this: u32, replacement: u32, copied_flag: u32, toggle_flag: u32) -> u32 {
    const OBJECT_SLOT: u32 = 0x204;
    const COPIED_FLAG: u32 = 0x210;
    const TOGGLED_FLAG: u32 = 0x216;
    unsafe {
        let slot = this.wrapping_add(OBJECT_SLOT);
        let old_object = (slot as *const u32).read_unaligned();
        if old_object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, old_object, slot);
        }
        (slot as *mut u32).write_unaligned(replacement);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, replacement, slot);
        (this.wrapping_add(COPIED_FLAG) as *mut u8).write(copied_flag as u8);
        let old_flags = (this.wrapping_add(TOGGLED_FLAG) as *const u8).read();
        let changed_bit = (old_flags ^ toggle_flag as u8) & 1;
        (this.wrapping_add(TOGGLED_FLAG) as *mut u8).write(old_flags ^ changed_bit);
    }
    0
});
