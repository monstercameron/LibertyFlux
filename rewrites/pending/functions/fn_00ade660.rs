// original: 0x00ade660 hash_find_or_insert (proposed)

/// Find a key in a chained hash table or insert a new node for it.
///
/// `this` is the table: bucket array pointer at `+0x0`, 16-bit bucket
/// count at `+0x4`, 16-bit used count at `+0x6`. `keyrec` points to a
/// record whose first word is the key. The bucket index is
/// `key % count`; the chain is walked through the link at node `+0x8`
/// comparing the word at node `+0x0` with the key. On a hit the
/// matching node is returned. On a miss the used count grows by one; a
/// table that becomes full calls the random callee (id 2) and the
/// rehash callee (id 3) first and recomputes the bucket. A 12-byte node
/// is then allocated through the malloc callee (id 4) and chained at
/// the bucket head (a null result is stored as the head). Returns the
/// node address plus four in all cases.
///
/// Edge cases: a zero count calls the assert callee (id 1) and then
/// divides by zero; the contract keeps the count nonzero and exempts
/// that callee. A failed allocation stores a null head and returns 4.
///
/// Original: thiscall, one stack word. Callee id 1 is cdecl with two
/// arguments, id 2 cdecl with one, id 3 thiscall with one, id 4 cdecl
/// with one.
lf_checker_rt::export!(thiscall, rw_00ade660(this: u32, keyrec: u32) -> u32 {
    unsafe {
        const BUCKETS: u32 = 0x0;
        const COUNT: u32 = 0x4;
        const USED: u32 = 0x6;
        const NODE_KEY: u32 = 0x0;
        const NODE_VAL: u32 = 0x4;
        const NODE_NEXT: u32 = 0x8;
        const NODE_SIZE: u32 = 12;
        const VALUE_OFF: u32 = 4;
        let count = ((this + COUNT) as *const u16).read_unaligned() as u32;
        if count == 0 {
            lf_checker_rt::callee_cdecl!(1, u32, 1, 0x0a);
        }
        let count = ((this + COUNT) as *const u16).read_unaligned() as u32;
        let key = (keyrec as *const u32).read_unaligned();
        let mut bucket = key % count;
        let table = ((this + BUCKETS) as *const u32).read_unaligned();
        let mut node = ((table + bucket * 4) as *const u32).read_unaligned();
        let mut hit = false;
        if node != 0 {
            loop {
                if ((node + NODE_KEY) as *const u32).read_unaligned() == key {
                    hit = true;
                    break;
                }
                node = ((node + NODE_NEXT) as *const u32).read_unaligned();
                if node == 0 {
                    break;
                }
            }
        }
        if !hit {
            let used = ((this + USED) as *const u16).read_unaligned().wrapping_add(1);
            ((this + USED) as *mut u16).write_unaligned(used);
            if used as u32 == count {
                let r: u32 = lf_checker_rt::callee_cdecl!(2, u32, count);
                let r16 = (r as u16) as u32;
                lf_checker_rt::callee_thiscall!(3, u32, this, r16);
                let count2 = ((this + COUNT) as *const u16).read_unaligned() as u32;
                bucket = key % count2;
            }
            let table2 = ((this + BUCKETS) as *const u32).read_unaligned();
            let old = ((table2 + bucket * 4) as *const u32).read_unaligned();
            let mem: u32 = lf_checker_rt::callee_cdecl!(4, u32, NODE_SIZE);
            let fresh = if mem != 0 {
                (mem as *mut u32).write_unaligned(key);
                ((mem + NODE_VAL) as *mut u32).write_unaligned(0);
                ((mem + NODE_NEXT) as *mut u32).write_unaligned(old);
                mem
            } else {
                0
            };
            let table3 = ((this + BUCKETS) as *const u32).read_unaligned();
            ((table3 + bucket * 4) as *mut u32).write_unaligned(fresh);
            node = fresh;
        }
        node.wrapping_add(VALUE_OFF)
    }
});
