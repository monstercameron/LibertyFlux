// original: 0x005EE5B0 copy_render_state_and_dispatch

/// Copy the defined render-state fields from `source` to the destination.
///
/// ECX is the destination and the one stack argument is the source; the
/// original returns the destination in EAX and removes that argument. It
/// copies 32-bit words at offsets 0x00..0x10, 0x1c..0x24 and 0x2c..0x40,
/// bytes at 0x14..0x1a, 0x28..0x29 and 0x44, and deliberately leaves padding
/// bytes 0x1b, 0x2a and 0x2b untouched. Loads and stores proceed in source order so
/// overlapping buffers retain the original's forward-copy behavior.
lf_checker_rt::export!(thiscall, rw_005ee5b0(destination: *mut u8, source: *const u8) -> *mut u8 {
    const FIRST_WORD_END: usize = 0x14;
    const FIRST_BYTE_START: usize = 0x14;
    const FIRST_BYTE_END: usize = 0x1b;
    const SECOND_WORD_START: usize = 0x1c;
    const SECOND_WORD_END: usize = 0x28;
    const SECOND_BYTE_START: usize = 0x28;
    const SECOND_BYTE_END: usize = 0x2a;
    const THIRD_WORD_START: usize = 0x2c;
    const THIRD_WORD_END: usize = 0x44;
    const FINAL_BYTE_OFFSET: usize = 0x44;

    unsafe {
        for offset in (0..FIRST_WORD_END).step_by(4) {
            let word = source.add(offset).cast::<u32>().read_unaligned();
            destination.add(offset).cast::<u32>().write_unaligned(word);
        }
        for offset in FIRST_BYTE_START..FIRST_BYTE_END {
            let byte = source.add(offset).read();
            destination.add(offset).write(byte);
        }
        for offset in (SECOND_WORD_START..SECOND_WORD_END).step_by(4) {
            let word = source.add(offset).cast::<u32>().read_unaligned();
            destination.add(offset).cast::<u32>().write_unaligned(word);
        }
        for offset in SECOND_BYTE_START..SECOND_BYTE_END {
            let byte = source.add(offset).read();
            destination.add(offset).write(byte);
        }
        for offset in (THIRD_WORD_START..THIRD_WORD_END).step_by(4) {
            let word = source.add(offset).cast::<u32>().read_unaligned();
            destination.add(offset).cast::<u32>().write_unaligned(word);
        }
        let final_byte = source.add(FINAL_BYTE_OFFSET).read();
        destination.add(FINAL_BYTE_OFFSET).write(final_byte);
    }
    destination
});
