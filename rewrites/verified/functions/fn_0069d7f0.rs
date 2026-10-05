// original: 0x0069D7F0 rage::crAnimChannelRleInt::compress

/// Compresses sample ints into an RLE channel, choosing the shift.
///
/// `this` is the channel object (value array at `+8`, bit stream at
/// `+0x10`, shift byte at `+0x18`); the stack arguments are the sample
/// words and the signed `count`. A count below 2 (signed) returns 0. Else
/// a run-length scan appends each run's value to the channel array and its
/// length to a temporary frame array (callees 1 and 2, thiscall: array,
/// `0x10`, answering the next slot word; the temporary array's register
/// argument is a frame pointer, so it is dropped from the comparison and
/// the struct contents snapshotted instead). Six candidate widths
/// (4 to 128) are then costed over the true runs and the cheapest (unsigned
/// compare, first wins ties) sets the shift byte to its bit length minus
/// one. The old bit stream, when non-null, is released through the thread
/// allocator reached as `tls[0] -> [+8] -> vtable[+0xC]` (callee 3,
/// thiscall), and the finalizer (callee 4, thiscall: stream slot, temporary
/// array; frame pointer skipped, struct snapshotted) consumes the runs; a
/// set word past the temporary count frees the temporary buffer through the
/// same allocator. Returns 1 in `al`.
///
/// Original: 0x0069D7F0 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_0069D7F0(this: u32, samples: u32, count: u32) -> u32 {
    unsafe {
        const ARR_OFF: u32 = 8;
        const BITS_OFF: u32 = 0x10;
        const SHIFT_OFF: u32 = 0x18;
        const TLS_SLOT: usize = 0;
        const HEAPOBJ_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        const APPEND_V: u32 = 1;
        const APPEND_T: u32 = 2;
        const FREE: u32 = 3;
        const FINALIZE: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if (count as i32) < 2 {
            return 0;
        }
        let mut tmp = [0u32, 0u32, 0u32];
        let tmpp = tmp.as_mut_ptr() as u32;
        let mut cur = rd32(samples);
        let mut run: u32 = 1;
        let mut i: u32 = 1;
        if (count as i32) > 1 {
            loop {
                let v = rd32(samples.wrapping_add(i.wrapping_mul(4)));
                if v != cur {
                    let slot = lf_checker_rt::callee_thiscall!(APPEND_V, u32, this + ARR_OFF, 0x10);
                    wr32(slot, cur);
                    let slot2 = lf_checker_rt::callee_thiscall!(APPEND_T, u32, tmpp, 0x10);
                    wr32(slot2, run);
                    cur = v;
                    run = 1;
                } else {
                    run = run.wrapping_add(1);
                }
                i = i.wrapping_add(1);
                if !((i as i32) < (count as i32)) {
                    break;
                }
            }
        }
        let slot = lf_checker_rt::callee_thiscall!(APPEND_V, u32, this + ARR_OFF, 0x10);
        wr32(slot, cur);
        let slot2 = lf_checker_rt::callee_thiscall!(APPEND_T, u32, tmpp, 0x10);
        wr32(slot2, run);
        let n = tmp[1] & 0xFFFF;
        let tptr = tmp[0];
        let mut best = 0xFFFFFFFFu32;
        let mut bestw = 0u32;
        let mut width = 4u32;
        let mut left = 6u32;
        loop {
            let mut e = width;
            let mut c = 0u32;
            loop {
                c += 1;
                e >>= 1;
                if e == 0 {
                    break;
                }
            }
            c -= 1;
            let mut cost = 0u32;
            if (n as i32) > 0 {
                let mut k = 0u32;
                while (k as i32) < (n as i32) {
                    let r = rd32(tptr.wrapping_add(k.wrapping_mul(4)));
                    let ar = if (r as i32) < 0 { (r as i32).wrapping_neg() as u32 } else { r };
                    let a = ar >> (c & 31);
                    cost = cost.wrapping_add(1).wrapping_add(a.wrapping_add(c));
                    if r != 0 {
                        cost = cost.wrapping_add(1);
                    }
                    k += 1;
                }
            }
            if cost < best {
                best = cost;
                bestw = width;
            }
            width = width.rotate_left(1);
            left -= 1;
            if left == 0 {
                break;
            }
        }
        let mut al = 0u32;
        let mut w = bestw;
        loop {
            al += 1;
            w >>= 1;
            if w == 0 {
                break;
            }
        }
        unsafe { ((this + SHIFT_OFF) as *mut u8).write((al - 1) as u8) };
        let old = rd32(this + BITS_OFF);
        if old != 0 {
            let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
            let heap_obj = rd32(tls_base + HEAPOBJ_OFF);
            let vtable = rd32(heap_obj);
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
            free(heap_obj, old);
        }
        let _ = lf_checker_rt::callee_thiscall!(FINALIZE, u32, this + BITS_OFF, tmpp);
        if rd16(tmpp + 6) != 0 {
            let p = tmp[0];
            if p != 0 {
                let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
                let heap_obj = rd32(tls_base + HEAPOBJ_OFF);
                let vtable = rd32(heap_obj);
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
                free(heap_obj, p);
            }
        }
        1
    }
});
