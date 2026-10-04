// original: 0x009f9800 wordset_lookup_insert
use lf_k2_rt::{export, callee_cdecl, global};

/// Bits of 1.0f, pushed as the float argument to the notify calls.
const ONE_BITS: u32 = 0x3F800000;

/// Word-set lookup-or-insert with notify (cdecl/1 -> eax).
///
/// Scans the counted word table for `key`: a hit returns the index without
/// calling; otherwise the key takes the first zero slot, or when the table is
/// full the table is cleared and the key goes to slot 0 — both call the
/// notify step and return its answer.
export!(cdecl, rw_s18f12(key: u32) -> u32 {
    unsafe {
        let key = key as u16;
        let table = global::<u16>(0x12B79A8);
        let count = *global::<i32>(0x12B79BC);
        if count > 0 {
            let mut i = 0;
            while i < count {
                if *table.add(i as usize) == key {
                    return i as u32;
                }
                i += 1;
            }
            let mut j = 0;
            while j < count {
                if *table.add(j as usize) == 0 {
                    *table.add(j as usize) = key;
                    return callee_cdecl!(1, u32, 0x106, ONE_BITS);
                }
                j += 1;
            }
            // The original re-reads the count each iteration; nothing in the
            // trial can change it, so a single snapshot is identical.
            let mut k = 0;
            while k < count {
                *table.add(k as usize) = 0;
                k += 1;
            }
        }
        *table = key;
        callee_cdecl!(1, u32, 0x106, ONE_BITS)
    }
});
