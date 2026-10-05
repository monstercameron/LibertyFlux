// original: 0x00cc2eb0 ped_task_state_init (proposed)

/// Zero a 73-word task-state object, then mark every slot's two handle fields invalid.
///
/// `this` points to the object: one header word followed by nine 8-word
/// records. Words are counted from `this`: word 0 is the header, words
/// `8*k+1 ..= 8*k+8` are record `k` (k = 0..9, covering offsets 0x4..0x120).
/// Every word is first set to 0; then the first two words of each record
/// (offsets 0x4/0x8, 0x24/0x28, ... 0x104/0x108) are set to 0xFFFF_FFFF,
/// the invalid-handle sentinel. The original writes the same 73 words in a
/// scattered order with the header last; only the final contents are
/// observable, so this rewrite fills them in order.
///
/// Original: 0x00cc2eb0 (thiscall, no stack arguments, no return value set,
/// no calls, no globals, no floating point).
lf_checker_rt::export!(thiscall, rw_00cc2eb0(this: u32) -> u32 {
    unsafe {
        /// Total words in the object (offsets 0x0..=0x120).
        const WORDS: usize = 73;
        /// Words per record after the header.
        const RECORD_WORDS: usize = 8;
        /// Records in the object.
        const RECORDS: usize = 9;
        /// Invalid-handle sentinel written into each record's first two words.
        const INVALID: u32 = 0xFFFF_FFFF;

        let base = this as *mut u32;
        let mut i = 0usize;
        while i < WORDS {
            base.add(i).write_unaligned(0);
            i += 1;
        }
        let mut k = 0usize;
        while k < RECORDS {
            base.add(RECORD_WORDS * k + 1).write_unaligned(INVALID);
            base.add(RECORD_WORDS * k + 2).write_unaligned(INVALID);
            k += 1;
        }
        0
    }
});
