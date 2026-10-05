// original: 0x00c0a880 stream_bucket_detach (proposed)

/// Detach the key's nodes from its bucket run and hand each to the helper.
///
/// `key` names the owner value and `index` selects a bucket from the table at
/// `TABLE` (8 bytes per entry). When the tag word at `TAGS` plus bucket times
/// `TAG_STRIDE` does not hold `index`, `index` is returned at once. Otherwise
/// each bucket from there on while its tag still holds `index` and it stays
/// below the bound at `BOUND` has its node chain (head at `HEADS` plus bucket
/// times `TAG_STRIDE`) walked: every node whose link at `LINK` equals `key`
/// has that link cleared and goes to the helper (callee 1) with the head
/// word's address. Returns `index` on every path.
///
/// Original: 0x00c0a880 (thiscall, two stack words; helper is thiscall).
lf_checker_rt::export!(thiscall, rw_00c0a880(this: u32, key: u32, index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x16cebc0;
        const TAGS: u32 = 0x16c8fb8;
        const HEADS: u32 = 0x16c8ff4;
        const HEADOFF: u32 = 0x16c8ff0;
        const TAG_STRIDE: u32 = 68;
        const BOUND: u32 = 0x16c8fb4;
        const LINK: u32 = 0x0c;
        const HELPER: u32 = 1;
        let base = lf_checker_rt::relocated(TABLE);
        let mut bucket =
            ((base.wrapping_add(index.wrapping_mul(8))) as *const u32).read_unaligned();
        let tags = lf_checker_rt::relocated(TAGS);
        if ((tags.wrapping_add(bucket.wrapping_mul(TAG_STRIDE))) as *const u32).read_unaligned() != index {
            return index;
        }
        let heads = lf_checker_rt::relocated(HEADS);
        let headoff = lf_checker_rt::relocated(HEADOFF);
        let bound = lf_checker_rt::relocated(BOUND);
        while (bucket as i32) < ((bound as *const u32).read_unaligned() as i32) {
            let mut node =
                ((heads.wrapping_add(bucket.wrapping_mul(TAG_STRIDE))) as *const u32).read_unaligned();
            while node != 0 {
                let next = (node as *const u32).read_unaligned();
                if ((node.wrapping_add(LINK)) as *const u32).read_unaligned() == key {
                    (node.wrapping_add(LINK) as *mut u32).write_unaligned(0);
                    let hp = headoff.wrapping_add(bucket.wrapping_mul(TAG_STRIDE));
                    let _r: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this, node, hp);
                }
                node = next;
            }
            bucket = bucket.wrapping_add(1);
            if ((tags.wrapping_add(bucket.wrapping_mul(TAG_STRIDE))) as *const u32).read_unaligned() != index {
                break;
            }
        }
        index
    }
});
