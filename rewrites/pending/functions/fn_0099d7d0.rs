// original: 0x0099d7d0 intro_sort_16
/// Introspective sort over 16-byte records keyed by the dword at offset 0xc.
///
/// Ranges of 0x100 bytes or fewer (low nibble masked, compared signed) are
/// left alone. Larger ranges run a quicksort loop: pick the median of the
/// first, middle and last keys, copy that 16-byte record to a stack slot,
/// partition through callee 1, recurse on the upper part and continue with
/// the lower part. When the depth budget runs out, callee 2 sorts the whole
/// range instead. The original also writes its decremented depth back into
/// its own incoming stack slot; that write is dead (no caller re-reads it)
/// and is not reproduced, so the contract disables the stack check.
export!(cdecl, rw_0099d7d0(lo: u32, hi: u32, _aux2: u32, depth: u32, aux: u32) -> () {
    unsafe {
        const SMALL: i32 = 0x100;
        if ((hi.wrapping_sub(lo) & 0xFFFF_FFF0) as i32) <= SMALL {
            return;
        }
        let mut hi = hi;
        let mut depth = depth;
        loop {
            if depth == 0 {
                callee_cdecl!(2, u32, lo, hi, hi, aux);
                return;
            }
            depth -= 1;
            let first_key = *(lo.wrapping_add(0x0c) as *const u32);
            let last_key = *(hi.wrapping_sub(4) as *const u32);
            let count = (hi.wrapping_sub(lo) as i32) >> 4;
            let mid = lo.wrapping_add(((count.wrapping_sub(count >> 31) >> 1) << 4) as u32);
            let mid_key = *(mid.wrapping_add(0x0c) as *const u32);
            let last_rec = hi.wrapping_sub(0x10);
            let pivot = if first_key < mid_key {
                if mid_key < last_key {
                    mid
                } else if first_key < last_key {
                    last_rec
                } else {
                    lo
                }
            } else if first_key < last_key {
                lo
            } else if mid_key < last_key {
                last_rec
            } else {
                mid
            };
            let mut tmp = [0u32; 4];
            let src = pivot as *const u32;
            tmp[0] = *src;
            tmp[1] = *src.add(1);
            tmp[2] = *src.add(2);
            tmp[3] = *src.add(3);
            // Partition takes the pivot copy in ECX plus the range, the
            // pivot words and the auxiliary value on the stack, and pops
            // nothing (the caller cleans): a thiscall-shaped call with cdecl
            // cleanup, modelled by the checker's noclean option.
            let partition: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            let part = partition(tmp.as_ptr() as u32, lo, hi, tmp[0], tmp[1], tmp[2], tmp[3], aux);
            rw_0099d7d0(part, hi, 0, depth, aux);
            hi = part;
            if ((part.wrapping_sub(lo) & 0xFFFF_FFF0) as i32) <= SMALL {
                return;
            }
        }
    }
});
