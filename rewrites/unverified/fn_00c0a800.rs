// original: 0x00c0a800 stream_bucket_sum (proposed)

/// Sum the worker's answers over the item's bucket run.
///
/// `this` guards the run through the gate (callee 1, no arguments): a set
/// low byte in its answer returns 0 at once. Otherwise the tag byte at `TAG`
/// past `item` selects a bucket index from the table at `TABLE` (8 bytes per
/// entry), and while the tag word at `TAGS` plus index times `TAG_STRIDE`
/// still holds the tag and the index stays below the bound at `BOUND` (signed
/// compares), the worker (callee 2) receives the tag word's address and the
/// item, and its answer is added to the total. Returns the total.
///
/// Original: 0x00c0a800 (thiscall, one stack word; both callees thiscall).
lf_checker_rt::export!(thiscall, rw_00c0a800(this: u32, item: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x4d;
        const TABLE: u32 = 0x16cebc0;
        const TAGS: u32 = 0x16c8fb8;
        const TAG_STRIDE: u32 = 68;
        const BOUND: u32 = 0x16c8fb4;
        const GATE: u32 = 1;
        const WORKER: u32 = 2;
        let g: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if g & 0xff != 0 {
            return 0;
        }
        let tag = (item.wrapping_add(TAG) as *const u8).read() as u32;
        let base = lf_checker_rt::relocated(TABLE);
        let mut idx = ((base.wrapping_add(tag.wrapping_mul(8))) as *const u32).read_unaligned();
        let tags = lf_checker_rt::relocated(TAGS);
        let bound = lf_checker_rt::relocated(BOUND);
        let mut total = 0u32;
        if ((tags.wrapping_add(idx.wrapping_mul(TAG_STRIDE))) as *const u32).read_unaligned() != tag {
            return 0;
        }
        while (idx as i32) < ((bound as *const u32).read_unaligned() as i32) {
            let tp = tags.wrapping_add(idx.wrapping_mul(TAG_STRIDE));
            let a: u32 = lf_checker_rt::callee_thiscall!(WORKER, u32, this, tp, item);
            total = total.wrapping_add(a);
            idx = idx.wrapping_add(1);
            if ((tags.wrapping_add(idx.wrapping_mul(TAG_STRIDE))) as *const u32).read_unaligned() != tag {
                break;
            }
        }
        total
    }
});
