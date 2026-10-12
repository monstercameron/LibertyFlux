// original: 0x005d4ef0 CPhysical::set_field_200_vector
/// Copy a four-word record into the physical object's field at offsets
/// 0x200 through 0x20c. The middle two words are float components and are
/// copied bit for bit; no arithmetic is performed. Return the final word.
/// The method takes `this` in ECX and one record pointer on the stack, which
/// it pops before returning.
lf_checker_rt::export!(thiscall, rw_005d4ef0(this: u32, record: u32) -> u32 {
    const RECORD_X_OFFSET: u32 = 4;
    const RECORD_Y_OFFSET: u32 = 8;
    const RECORD_TAIL_OFFSET: u32 = 12;
    const OBJECT_FIELD_OFFSET: u32 = 0x200;

    unsafe {
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let first_word = read_u32(record);
        let x_bits = read_u32(record.wrapping_add(RECORD_X_OFFSET));
        let y_bits = read_u32(record.wrapping_add(RECORD_Y_OFFSET));
        write_u32(this.wrapping_add(OBJECT_FIELD_OFFSET), first_word);
        write_u32(this.wrapping_add(OBJECT_FIELD_OFFSET + 4), x_bits);
        write_u32(this.wrapping_add(OBJECT_FIELD_OFFSET + 8), y_bits);
        let tail_word = read_u32(record.wrapping_add(RECORD_TAIL_OFFSET));
        write_u32(this.wrapping_add(OBJECT_FIELD_OFFSET + 12), tail_word);
        tail_word
    }
});
