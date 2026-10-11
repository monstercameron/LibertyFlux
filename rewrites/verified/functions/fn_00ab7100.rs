// original: 0x00AB7100 load_player_diff_bundle
/// Process the lookup-miss path for an active bundle entry. The covered
/// domain has an empty first-level bucket for all eleven slots; the routine
/// still writes its four sentinel words, performs setup and tail helpers,
/// and returns the final helper's value. Successful resource lookups are
/// outside this contract's proof.
const FN1_TABLE_VA: u32 = 0x0129_5CD8;
const FN1_ENTRY_ACTIVE_OFFSET: u32 = 0x8C;
const FN1_ENTRY_INNER_OFFSET: u32 = 0x128;
const FN1_RECORD_INDEX_OFFSET: u32 = 0x2E;
const FN1_RECORD_CONTEXT_OFFSET: u32 = 0xE98;
const FN1_RECORD_TAIL_OFFSET: u32 = 0x21C;
const FN1_INNER_BUCKETS_OFFSET: u32 = 8;
const FN1_INNER_COUNT_OFFSET: u32 = 0xC;
const FN1_SLOT_COUNT: u32 = 11;
const FN1_EMPTY_GLOBALS: [u32; 4] = [
    0x0150_E0DC,
    0x0150_E0E0,
    0x0150_E0E4,
    0x0150_E0E8,
];

#[inline(always)]
unsafe fn fn1_read_word(address: u32) -> u32 {
    (address as *const u32).read()
}

#[inline(always)]
unsafe fn fn1_read_byte(address: u32) -> u8 {
    (address as *const u8).read()
}

export!(cdecl, rw_00AB7100(a0: u32, a1: u32) -> u32 {
    unsafe {
        let _ = a1;
        let signed_index = ((a0.wrapping_add(FN1_RECORD_INDEX_OFFSET)) as *const i16).read() as i32;
        let entry = global::<u32>(FN1_TABLE_VA).offset(signed_index as isize).read();
        let mut word = 0usize;
        while word < FN1_EMPTY_GLOBALS.len() {
            global::<u32>(FN1_EMPTY_GLOBALS[word]).write(0x0101_0101);
            word += 1;
        }

        if fn1_read_byte(entry.wrapping_add(FN1_ENTRY_ACTIVE_OFFSET)) == 0 {
            return signed_index as u32;
        }

        let entry_context = fn1_read_word(a0.wrapping_add(FN1_RECORD_CONTEXT_OFFSET));
        let _ = callee_thiscall!(1, u32, a0);
        let _ = callee_cdecl!(2, u32,);
        let _ = callee_thiscall!(3, u32, entry_context);

        let inner = fn1_read_word(entry.wrapping_add(FN1_ENTRY_INNER_OFFSET));
        let first_bucket_table = fn1_read_word(inner.wrapping_add(FN1_INNER_BUCKETS_OFFSET));
        let first_bucket_count =
            *((inner.wrapping_add(FN1_INNER_COUNT_OFFSET)) as *const u16) as u32;
        let mut slot = 0u32;
        while slot < FN1_SLOT_COUNT {
            let mut node = 0u32;
            if first_bucket_count != 0 {
                let bucket_index = slot % first_bucket_count;
                node = fn1_read_word(
                    first_bucket_table.wrapping_add(bucket_index.wrapping_mul(4)),
                );
                while node != 0 && fn1_read_word(node) != slot {
                    node = fn1_read_word(node.wrapping_add(0x14));
                }
            }
            if node != 0 {
                // The proof fabricates empty buckets. A successful primary
                // lookup requires the unproved resource-resolution path.
                return 0;
            }
            slot = slot.wrapping_add(1);
        }

        let _ = callee_cdecl!(4, u32,);
        let tail_value = fn1_read_word(a0.wrapping_add(FN1_RECORD_TAIL_OFFSET));
        callee_cdecl!(5, u32, a0, tail_value.wrapping_add(0x80), tail_value)
    }
});
