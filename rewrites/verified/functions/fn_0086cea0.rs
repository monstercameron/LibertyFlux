// original: 0x0086CEA0 Script_NativeWriteback

/// Copy the pending native writeback records to their destination buffers.
///
/// The thiscall object has a 32-bit count at byte offset 12, destination
/// pointers starting at byte offset 16, and source triples in 16-byte
/// records starting at byte offset 32. The routine processes records in
/// descending index order, copies three words to each destination, and
/// decrements the count before each copy. Its final decrement leaves the
/// count at `u32::MAX`, including when the initial count is zero. The
/// function has no meaningful return value and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_0086CEA0(context: u32) -> () {
    const COUNT_WORD: usize = 3;
    const DESTINATION_BASE_WORD: usize = 4;
    const SOURCE_BASE_WORD: usize = 8;
    const SOURCE_RECORD_WORDS: usize = 4;
    const COPY_WORDS: usize = 3;

    unsafe {
        let object = context as *mut u32;
        let count = object.add(COUNT_WORD);
        let mut remaining = count.read_unaligned();
        if remaining == 0 {
            count.write_unaligned(u32::MAX);
            return;
        }

        loop {
            remaining = remaining.wrapping_sub(1);
            count.write_unaligned(remaining);

            let destination_pointer = object
                .add(DESTINATION_BASE_WORD + remaining as usize)
                .read_unaligned();
            let destination = destination_pointer as *mut u32;
            let source_word = SOURCE_BASE_WORD
                .wrapping_add((remaining as usize).wrapping_mul(SOURCE_RECORD_WORDS));
            for field in 0..COPY_WORDS {
                let value = object.add(source_word + field).read_unaligned();
                destination.add(field).write_unaligned(value);
            }

            if remaining == 0 {
                break;
            }
        }
        count.write_unaligned(u32::MAX);
    }
});
