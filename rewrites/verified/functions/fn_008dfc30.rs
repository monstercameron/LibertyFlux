// original: 0x008dfc30 resize_bucket_table

/// Resize the table's hash buckets to `count` slots: allocate the new table,
/// clear it, rehash every node of the old table into it by `key % count`
/// (chaining through each node's `+8` link), then free the old table and
/// publish the new table and count.
///
/// A zero tag byte at `this + 0xb` returns early with no work. The multiply
/// that sizes the allocation cannot overflow: `count` arrives as a 16-bit
/// value, so the overflow arm of the size computation is dead by
/// construction. The old count is a 16-bit word at `this + 4`.
///
/// Original: thiscall with one stack argument, callee cleanup; returns the
/// free call's answer (0 on the early path, where the original returns
/// whatever the entry `eax` held).
lf_checker_rt::export!(thiscall, rw_008dfc30(this: *mut u8, count: u32) -> u32 {
    unsafe {
        if *this.add(0xb) == 0 {
            return 0;
        }
        let n = (count as u16) as u32;
        let newtab =
            lf_checker_rt::callee_cdecl!(1, u32, n.wrapping_mul(4)) as *mut u32;
        if n != 0 {
            let mut i = 0u32;
            while i < n {
                *newtab.add(i as usize) = 0;
                i += 1;
            }
        }
        let w = this as *mut u32;
        let oldtab = *w as *mut u32;
        let oldcount = *(this.add(4) as *const u16) as u32;
        if oldcount != 0 {
            let mut i = 0u32;
            while i < oldcount {
                let head = *oldtab.add(i as usize);
                if head != 0 {
                    let mut cur = head as *mut u32;
                    loop {
                        let key = *cur;
                        let slot = (key % n) as usize;
                        let next = *cur.add(2);
                        *cur.add(2) = *newtab.add(slot);
                        *newtab.add(slot) = cur as u32;
                        if next == 0 {
                            break;
                        }
                        cur = next as *mut u32;
                    }
                }
                i += 1;
            }
        }
        let ans = lf_checker_rt::callee_cdecl!(2, u32, oldtab as u32);
        *(this.add(4) as *mut u16) = count as u16;
        *w = newtab as u32;
        ans
    }
});
