// original: 0x00adec20 hash_multimap_insert (proposed)

/// Append a value to one sub-list per set bit of a mask.
///
/// `this` is the table base, `val` the value, `mask` a bit mask, `base`
/// a sub-list base index. A zero mask returns at once. Otherwise, for
/// each set bit from high to low, the value is appended to the record
/// at `this + (base + 9*bit)*8 + 0x140`: a vector of `{array, u16
/// count, u16 capacity}` that grows by 16 slots through the malloc
/// callee (id 1) and the free callee (id 2) whenever it is full, then
/// takes the value at index `count` and grows the count by one.
///
/// Edge cases: bits are visited high to low; a full record with a
/// zero count still reallocates; the mask walk is the original's
/// doubling loop, exact for all inputs.
///
/// Original: thiscall, three stack words, two direct callees (id 1
/// and id 2, cdecl with one argument each). Returns nothing.
lf_checker_rt::export!(thiscall, rw_00adec20(this: u32, val: u32, mask: u32, base: u32) -> u32 {
    unsafe {
        const REC_OFF: u32 = 0x140;
        const REC_ARR: u32 = 0x0;
        const REC_COUNT: u32 = 0x4;
        const REC_CAP: u32 = 0x6;
        const GROW: u16 = 0x10;
        let mut b = mask;
        if b == 0 {
            return 0;
        }
        loop {
            // Highest set bit by the original's doubling walk; clears it.
            let mut edx: u32 = 0;
            let mut eax = b;
            if (b as i32) >= 0 {
                loop {
                    eax = eax.wrapping_add(eax);
                    edx = edx.wrapping_add(1);
                    eax |= 1;
                    if (eax as i32) < 0 {
                        break;
                    }
                }
            }
            let ecx = 0x1Fu32.wrapping_sub(edx);
            b = b.wrapping_add(0xFFFF_FFFFu32.wrapping_shl(ecx));
            let idx = base.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(ecx);
            let rec = this.wrapping_add(idx.wrapping_mul(8)).wrapping_add(REC_OFF);
            let cnt = ((rec + REC_COUNT) as *const u16).read_unaligned();
            let cap = ((rec + REC_CAP) as *const u16).read_unaligned();
            if cnt == cap {
                let ncap = cap.wrapping_add(GROW);
                ((rec + REC_CAP) as *mut u16).write_unaligned(ncap);
                let mem: u32 =
                    lf_checker_rt::callee_cdecl!(1, u32, (ncap as u32).wrapping_mul(4));
                let old = ((rec + REC_ARR) as *const u32).read_unaligned();
                let mut k: u32 = 0;
                if 0u16 < cnt {
                    loop {
                        let w = ((old + k.wrapping_mul(4)) as *const u32).read_unaligned();
                        ((mem + k.wrapping_mul(4)) as *mut u32).write_unaligned(w);
                        k = k.wrapping_add(1);
                        if !((k as u16) < cnt) {
                            break;
                        }
                    }
                }
                lf_checker_rt::callee_cdecl!(2, u32, old);
                ((rec + REC_ARR) as *mut u32).write_unaligned(mem);
            }
            let cnt2 = ((rec + REC_COUNT) as *const u16).read_unaligned() as u32;
            let arr = ((rec + REC_ARR) as *const u32).read_unaligned();
            ((rec + REC_COUNT) as *mut u16)
                .write_unaligned((cnt2 as u16).wrapping_add(1));
            ((arr + cnt2.wrapping_mul(4)) as *mut u32).write_unaligned(val);
            if b == 0 {
                break;
            }
        }
    }
    0
});
