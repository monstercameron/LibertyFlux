// original: 0x0099da10 heap_sift_up_subkey
/// Sift-up step complementing the sift-down: insert a record into a max-heap.
///
/// `idx` is both the starting hole and the sift key. The original reads the
/// key from the ESI register at entry, and its only caller in the whole image
/// always enters with ESI equal to this stack argument (verified by
/// disassembly plus an image-wide caller scan), so the rewrite takes the key
/// from the argument. The hole walks up while the parent key is below the key
/// and stops at `lo`; the record is stored, the security-cookie check runs
/// (stubbed by the checker) and its answer is returned.
export!(cdecl, rw_0099da10(arr: u32, idx: u32, lo: u32, v0: u32, v1: u32, v2: u32, v3: u32, _f: u32) -> u32 {
    unsafe {
        let mut hole = idx;
        if idx > lo {
            loop {
                let parent = hole.wrapping_sub(1) / 2;
                let pk = *((arr.wrapping_add(parent.wrapping_mul(16)).wrapping_add(12))
                    as *const u32);
                if pk >= idx {
                    break;
                }
                let d = arr.wrapping_add(hole.wrapping_mul(16)) as *mut u32;
                let s = arr.wrapping_add(parent.wrapping_mul(16)) as *const u32;
                *d = *s;
                *d.add(1) = *s.add(1);
                *d.add(2) = *s.add(2);
                *d.add(3) = *s.add(3);
                hole = parent;
                if hole <= lo {
                    break;
                }
            }
        }
        let d = arr.wrapping_add(hole.wrapping_mul(16)) as *mut u32;
        *d = v0;
        *d.add(1) = v1;
        *d.add(2) = v2;
        *d.add(3) = v3;
        callee_stdcall!(1, u32,)
    }
});
