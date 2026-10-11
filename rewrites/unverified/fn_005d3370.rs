// original: 0x005D3370 event_data_copy_by_tag

/// Copy one four-word event payload according to its leading type tag.
///
/// The type tag is the `u32` at `this`. Tag 5 selects the payload embedded at
/// `this + 0x10`; tag 4 selects `source`. Both copy four raw words into
/// `destination` and return one; all other tags leave memory unchanged and
/// return zero. The values are copied without numeric float conversion. The
/// thiscall frame has two stack arguments and the callee removes eight bytes.
lf_checker_rt::export!(thiscall, rw_005d3370(this: u32, source: u32, destination: u32) -> u32 {
    unsafe {
        const FIRST_PAYLOAD_WORD: usize = 0;
        const EMBEDDED_PAYLOAD: u32 = 0x10;
        const LAST_PAYLOAD_WORD: usize = 3;
        const TAG_INLINE: u32 = 5;
        const TAG_POINTER: u32 = 4;

        let tag = (this as *const u32).read_unaligned();
        let source_address = match tag {
            TAG_INLINE => this.wrapping_add(EMBEDDED_PAYLOAD),
            TAG_POINTER => source,
            _ => return 0,
        };

        let source_words = source_address as *const u32;
        let destination_words = destination as *mut u32;
        // The original stages the first three loads before any store, then
        // reads the fourth word after those stores; preserve that overlap case.
        let first = source_words.add(FIRST_PAYLOAD_WORD).read_unaligned();
        let second = source_words.add(1).read_unaligned();
        let third = source_words.add(2).read_unaligned();
        destination_words.write_unaligned(first);
        destination_words.add(1).write_unaligned(second);
        destination_words.add(2).write_unaligned(third);
        let fourth = source_words.add(LAST_PAYLOAD_WORD).read_unaligned();
        destination_words.add(3).write_unaligned(fourth);
        1
    }
});
