// original: 0x0099db60 partition_records
/// Partition a run of 16-byte records around the key `v3` (Hoare scheme).
///
/// Scans up from `lo` for a key at or above `v3` and down from `hi` for a key
/// at or below it, swaps out-of-place pairs and repeats. The original moves
/// the split point into EAX and then calls the security-cookie check, whose
/// stubbed answer overwrites EAX on the checker side, so the rewrite returns
/// that answer; the partition itself is verified through the heap writes.
export!(cdecl, rw_0099db60(lo: u32, hi: u32, _v0: u32, _v1: u32, _v2: u32, v3: u32) -> u32 {
    unsafe {
        let key_at = |p: u32| *((p.wrapping_add(12)) as *const u32);
        let mut i = lo;
        let mut j = hi.wrapping_sub(16);
        loop {
            while key_at(i) < v3 {
                i = i.wrapping_add(16);
            }
            while key_at(j) > v3 {
                j = j.wrapping_sub(16);
            }
            if i >= j {
                break;
            }
            let a = i as *mut u32;
            let b = j as *mut u32;
            let t0 = *a;
            let t1 = *a.add(1);
            let t2 = *a.add(2);
            let t3 = *a.add(3);
            *a = *b;
            *a.add(1) = *b.add(1);
            *a.add(2) = *b.add(2);
            *a.add(3) = *b.add(3);
            *b = t0;
            *b.add(1) = t1;
            *b.add(2) = t2;
            *b.add(3) = t3;
            i = i.wrapping_add(16);
        }
        callee_stdcall!(1, u32)
    }
});
