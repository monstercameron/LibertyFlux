// original: 0x00ab17f0 stream_mark_cancelled (proposed)

/// Cancel every streaming request with id `id`, honouring `filter`.
///
/// Walks the bucket chain at `+0x143048`: in buckets whose tag at `+8`
/// equals `id`, releases each chained entry whose owner at `+0x4c` equals
/// `filter` (any owner when `filter` is zero) through the release callee.
/// Then scans the `+0x143050`-long array at `+0x143054` (stride `0x30`) and
/// sets the done byte at `+0x2e` of entries whose tag at `+0x24` equals
/// `id` with a matching owner. No return value.
///
/// Callees: 1 = entry release (thiscall, two words).
///
/// Original: 0x00ab17f0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00ab17f0(this: u32, id: u32, filter: u32) -> u32 {
    unsafe {
        const RELEASE: u32 = 1;
        const CHAIN_OFF: u32 = 0x143048;
        const COUNT_OFF: u32 = 0x143050;
        const ARRAY_OFF: u32 = 0x143054;
        const STRIDE: u32 = 0x30;
        const BUCKET_TAG: u32 = 8;
        const ENTRIES_OFF: u32 = 0xB8;
        const OWNER_OFF: u32 = 0x4C;
        const ENTRY_TAG: u32 = 0x24;
        const DONE_OFF: u32 = 0x2E;
        let mut bucket = ((this + CHAIN_OFF) as *const u32).read_unaligned();
        while bucket != 0 {
            if ((bucket + BUCKET_TAG) as *const u32).read_unaligned() == id {
                let mut entry = ((bucket + ENTRIES_OFF) as *const u32).read_unaligned();
                while entry != 0 {
                    let next = (entry as *const u32).read_unaligned();
                    if filter == 0
                        || ((entry + OWNER_OFF) as *const u32).read_unaligned() == filter
                    {
                        lf_checker_rt::callee_thiscall!(RELEASE, u32, this, bucket, entry);
                    }
                    entry = next;
                }
            }
            bucket = (bucket as *const u32).read_unaligned();
        }
        let n = ((this + COUNT_OFF) as *const u32).read_unaligned() as i32;
        let mut i = 0i32;
        while i < n {
            let e = (this + ARRAY_OFF).wrapping_add((i as u32).wrapping_mul(STRIDE));
            if ((e + ENTRY_TAG) as *const u32).read_unaligned() == id {
                if filter == 0 {
                    ((e + DONE_OFF) as *mut u8).write(1);
                } else {
                    let owner_ptr = (e as *const u32).read_unaligned();
                    if ((owner_ptr + OWNER_OFF) as *const u32).read_unaligned() == filter {
                        ((e + DONE_OFF) as *mut u8).write(1);
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        0
    }
});
