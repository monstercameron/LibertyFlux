// original: 0x00db1a00 UILayoutFrame::vf55
/// Refresh the frame's parameter block, allocating it on first use.
///
/// When the frame has no block yet, a fresh one is allocated and
/// initialized; when it already has one, the stored stamp must match a
/// freshly computed one or the refresh is skipped. A successful refresh
/// stores the new stamp, the count, the fraction bits and two zero words.
/// The result is the stamp used, or the comparison stamp when skipped.
export!(thiscall, rw_00db1a00(this_ptr: u32, fraction_bits: u32, count: u32) -> u32 {
    unsafe {
        const BLOCK_SLOT: usize = 0xec;
        const STAMP_SLOT: usize = 0x4c;
        const BLOCK_SIZE: u32 = 0x14;

        let obj = this_ptr as *const u8;
        let table = *(this_ptr as *const u32) as usize;
        let stamp_at = *((table + STAMP_SLOT) as *const u32) as usize;
        let stamp: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(stamp_at);

        let slot = obj.add(BLOCK_SLOT) as *mut u32;
        if *slot == 0 {
            let block = callee_cdecl!(1, u32, BLOCK_SIZE);
            if block == 0 {
                *slot = 0;
            } else {
                let words = block as *mut u32;
                *words.add(0) = 0;
                *words.add(1) = 1;
                *words.add(2) = 0;
                *words.add(3) = 0;
                *words.add(4) = 0;
                *slot = block;
            }
            let fresh = stamp(this_ptr);
            let words = *slot as *mut u32;
            // Same store order as the original, including which store
            // faults first when allocation failed.
            *words.add(4) = 0;
            *words.add(3) = 0;
            *words.add(2) = fresh;
            *words.add(1) = count;
            *words.add(0) = fraction_bits;
            fresh
        } else {
            let current = stamp(this_ptr);
            let words = *slot as *mut u32;
            if *words.add(2) == current {
                let fresh = stamp(this_ptr);
                *words.add(4) = 0;
                *words.add(3) = 0;
                *words.add(2) = fresh;
                *words.add(1) = count;
                *words.add(0) = fraction_bits;
                fresh
            } else {
                current
            }
        }
    }
});
