// original: 0x00c72330 initialize_animation_stream_segment

/// Copy two 16-byte source records into the object's header, store the low
/// 16 bits of `array_count` at `+0x20` and the low byte of `kind` at `+0x22`,
/// clear the word at `+0x4c`, then call the array helper with destination
/// `this+0x24`, `array_source`, and `array_count*4` bytes. The copied scalar
/// fields are bit-preserving 32-bit values, including the two float fields.
lf_checker_rt::export!(thiscall, rw_00c72330(
    this: u32,
    first_record: u32,
    second_record: u32,
    array_source: u32,
    array_count: u32,
    kind: u32
) -> u32 {
    const ARRAY_HELPER: u32 = 1;
    const RECORD_WORDS: u32 = 4;
    const ARRAY_DESTINATION: u32 = 0x24;
    const COUNT_FIELD: u32 = 0x20;
    const KIND_FIELD: u32 = 0x22;
    const RESET_FIELD: u32 = 0x4c;

    unsafe {
        for index in 0..RECORD_WORDS {
            let word = ((first_record.wrapping_add(index * 4)) as *const u32).read_unaligned();
            ((this.wrapping_add(index * 4)) as *mut u32).write_unaligned(word);
        }
        for index in 0..RECORD_WORDS {
            let word = ((second_record.wrapping_add(index * 4)) as *const u32).read_unaligned();
            ((this.wrapping_add(0x10 + index * 4)) as *mut u32).write_unaligned(word);
        }
        ((this.wrapping_add(COUNT_FIELD)) as *mut u16).write_unaligned(array_count as u16);
        ((this.wrapping_add(KIND_FIELD)) as *mut u8).write_unaligned(kind as u8);
        ((this.wrapping_add(RESET_FIELD)) as *mut u32).write_unaligned(0);
    }

    let _ = lf_checker_rt::callee_cdecl!(
        ARRAY_HELPER,
        u32,
        this.wrapping_add(ARRAY_DESTINATION),
        array_source,
        array_count.wrapping_mul(4)
    );
    this
});
