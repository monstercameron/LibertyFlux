// original: 0x0099d960 select_and_sift_16
/// Selection pass over 16-byte records keyed by the dword at offset 0xc.
///
/// First invokes callee 1 with the base, the cursor and the auxiliary value.
/// Then scans records from the cursor up to the end: any record whose key is
/// below the base record's key overwrites its slot with the base record,
/// while the evicted 16 bytes go through callee 2 together with the base, a
/// zero index, the record count and the auxiliary value. Finally it derives a
/// patched end pointer (the cursor with its low byte replaced by the
/// auxiliary value's low byte) and, while the span stays above 0x10 bytes,
/// invokes callee 3 with the base, a cursor walking down in 0x10 steps and
/// the patched end. The original also stores the patched low byte back into
/// its own incoming cursor slot; that write is dead (no caller re-reads it)
/// and is not reproduced, so the contract disables the stack check.
export!(cdecl, rw_0099d960(a0: u32, a1: u32, a2: u32, _a3: u32, a4: u32) -> () {
    unsafe {
        callee_cdecl!(1, u32, a0, a1, a4);
        let mut esi = a1;
        if a1 < a2 {
            loop {
                let ekey = *((esi.wrapping_add(0x0c)) as *const u32);
                let bkey = *((a0.wrapping_add(0x0c)) as *const u32);
                if ekey < bkey {
                    let s = esi as *const u32;
                    let b = a0 as *const u32;
                    let t0 = *s;
                    let t1 = *s.add(1);
                    let t2 = *s.add(2);
                    let t3 = *s.add(3);
                    let d = esi as *mut u32;
                    *d = *b;
                    *d.add(1) = *b.add(1);
                    *d.add(2) = *b.add(2);
                    *d.add(3) = *b.add(3);
                    let count = (a1.wrapping_sub(a0) as i32) >> 4;
                    callee_cdecl!(2, u32, a0, 0, count as u32, t0, t1, t2, t3, a4);
                }
                esi = esi.wrapping_add(0x10);
                if esi >= a2 {
                    break;
                }
            }
        }
        let span = a1.wrapping_sub(a0);
        if ((span & 0xFFFF_FFF0) as i32) <= 0x10 {
            return;
        }
        let patched = (a1 & 0xFFFF_FF00) | (a4 & 0xFF);
        let mut edi = a1;
        let mut left = span;
        loop {
            callee_cdecl!(3, u32, a0, edi, patched);
            left = left.wrapping_sub(0x10);
            edi = edi.wrapping_sub(0x10);
            if ((left & 0xFFFF_FFF0) as i32) <= 0x10 {
                break;
            }
        }
    }
});
