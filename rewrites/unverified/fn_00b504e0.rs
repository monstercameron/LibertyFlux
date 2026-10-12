// original: 0x00B504E0 route_copy_staged_to_active

/// Copy `count` staged 16-byte records into the active records beginning at
/// byte offset `0x10`. The source array begins at `0x118`; each destination
/// record receives two raw words and two float words. The source float at
/// byte offset `0x118` in each record is increased by the relocated global
/// float, with both
/// operands kept in source order. Finally, the original count is copied to
/// offset `0` and the staged count at `0x0c` is cleared. A zero count returns
/// byte zero without reading the global; a nonzero count returns byte one.
lf_checker_rt::export!(thiscall, rw_b504e0(this: *mut u8) -> u8 {
    const COUNT_OFFSET: usize = 0x0c;
    const DESTINATION_OFFSET: usize = 0x10;
    const SOURCE_OFFSET: usize = 0x118;
    const RECORD_STRIDE: usize = 0x10;
    const GLOBAL_INCREMENT_VA: u32 = 0x00FE8830;

    unsafe {
        let count = (this.add(COUNT_OFFSET) as *const u32).read_unaligned();
        if count == 0 {
            return 0;
        }
        let increment = lf_checker_rt::global::<f32>(GLOBAL_INCREMENT_VA).read_unaligned();
        for record_index in 0..count as usize {
            let source = this.add(SOURCE_OFFSET + record_index * RECORD_STRIDE);
            let destination = this.add(DESTINATION_OFFSET + record_index * RECORD_STRIDE);
            let leading_word = (source.sub(8) as *const u32).read_unaligned();
            let leading_float = (source.sub(4) as *const f32).read_unaligned();
            let source_value = (source as *const f32).read_unaligned();
            let trailing_word = (source.add(4) as *const u32).read_unaligned();
            let updated_value = core::hint::black_box(source_value) + core::hint::black_box(increment);
            (destination as *mut u32).write_unaligned(leading_word);
            (destination.add(4) as *mut f32).write_unaligned(leading_float);
            (destination.add(8) as *mut f32).write_unaligned(updated_value);
            (destination.add(12) as *mut u32).write_unaligned(trailing_word);
        }
        (this as *mut u32).write_unaligned(count);
        (this.add(COUNT_OFFSET) as *mut u32).write_unaligned(0);
        1
    }
});
