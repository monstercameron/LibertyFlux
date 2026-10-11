// original: 0x00C1FB30 streaming_clear_entry_flags

/// Clear the low three flag bits in the first word of the receiver and zero
/// its next word, then return the receiver pointer. The two fields are
/// unaligned-safe 32-bit values at offsets zero and four.
lf_checker_rt::export!(thiscall, rw_00c1fb30(this: u32) -> u32 {
    unsafe {
        const FLAGS_OFFSET: u32 = 0;
        const VALUE_OFFSET: u32 = 4;
        const FLAG_MASK: u32 = 0x7;
        let flags_ptr = this.wrapping_add(FLAGS_OFFSET) as *mut u32;
        let flags = flags_ptr.read_unaligned();
        flags_ptr.write_unaligned(flags & !FLAG_MASK);
        (this.wrapping_add(VALUE_OFFSET) as *mut u32).write_unaligned(0);
        this
    }
});
