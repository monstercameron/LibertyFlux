// original: 0x00888B90 stream_hash_regrow (proposed)

/// Regrow the word-keyed hash table to a new bucket count.
///
/// When the live byte at `+0xb` is set, allocates (callee 1) the new
/// bucket array (`new * 4` bytes; the multiply cannot overflow a word),
/// zeroes it, rehashes every node of every old bucket (`key % new`, link
/// at node `+8`, new head prepended), frees the old array (callee 2) and
/// publishes the new array and count. A zero new count skips the loops.
/// The answer is the free entry's answer.
///
/// The contract keeps the live byte set: the early return leaves `eax`
/// untouched, which a rewrite cannot reproduce.
///
/// Original: 0x00888B90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00888B90(this: u32, arg: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const NODE_KEY: u32 = 0x00;
        const NODE_NEXT: u32 = 0x08;
        const ALLOC: u32 = 1;
        const FREE: u32 = 2;
        let new = arg as u16 as u32;
        // `mul edx; seto` can never overflow for a word, so the size is
        // always `new * 4`.
        let size = new.wrapping_mul(4);
        let buckets = lf_checker_rt::callee_cdecl!(ALLOC, u32, size);
        if new != 0 {
            let mut i = 0u32;
            while i < new {
                ((buckets.wrapping_add(i.wrapping_mul(4))) as *mut u32)
                    .write_unaligned(0);
                i = i.wrapping_add(1);
            }
        }
        let count = ((this + COUNT) as *const u16).read_unaligned() as u32;
        let old = ((this + TABLE) as *const u32).read_unaligned();
        let mut b = 0u32;
        while b < count {
            let mut e = (((old.wrapping_add(b.wrapping_mul(4))))
                as *const u32)
                .read_unaligned();
            while e != 0 {
                let key =
                    ((e + NODE_KEY) as *const u16).read_unaligned() as u32;
                let nb = key % new;
                let next = ((e + NODE_NEXT) as *const u32).read_unaligned();
                let slot = buckets.wrapping_add(nb.wrapping_mul(4));
                let head = ((slot) as *const u32).read_unaligned();
                ((e + NODE_NEXT) as *mut u32).write_unaligned(head);
                ((slot) as *mut u32).write_unaligned(e);
                e = next;
            }
            b = b.wrapping_add(1);
        }
        let f = lf_checker_rt::callee_cdecl!(FREE, u32, old);
        ((this + COUNT) as *mut u16).write_unaligned(new as u16);
        ((this + TABLE) as *mut u32).write_unaligned(buckets);
        f
    }
});
