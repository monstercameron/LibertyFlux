// original: 0x0069BEF0 rage::crAnimChannelCurveFloat::compress

/// Compresses sample floats into a curve-float channel.
///
/// `this` is the channel object (element base at `+8`, element count word
/// at `+0xC`, segment count word at `+0xE`, range at `+0x10`, minimum at
/// `+0x14`); the stack arguments are the samples, the signed `count`, the
/// stride and the tolerance as `f32` bits. A count below 3 (signed) or a
/// zero tolerance (ordered equal; NaN continues) returns 0. Else the
/// strided samples are scanned for a maximum and minimum: each tracker
/// keeps the sample unless already past it (`ja` skips the store, so NaN
/// updates, being unordered), the maximum starting from a tiny seed and
/// the minimum from a huge one. The normalized buffer is allocated through
/// the thread allocator reached as `tls[0] -> [+8] -> vtable[+8]`
/// (callee 1, thiscall: allocator, bytes, `0x10`, `0`; the byte size
/// saturates on overflow) and filled with `(sample - min) / (max - min)`
/// in the original's order and loop shape (groups of four, then singles).
/// The segmenter (callee 2, thiscall: channel, buffer, 0, `count - 1`,
/// tolerance, out-struct; the out-struct pointer is skipped and its
/// contents snapshotted) classifies the buffer; a zero answer frees the
/// buffer and returns 0. Otherwise a key block is allocated (callee 4,
/// stdcall), each list node is copied into place (callee 6, thiscall:
/// destination, source; source contents snapshotted) and the node, its
/// source and the source's data are released through `vtable[+0xC]`
/// (callee 3, thiscall) unless null, the buffer is released the same way,
/// and the list is purged (callee 5, thiscall: list slot; register
/// argument dropped, contents snapshotted). The original reuses its dead
/// tolerance/stride argument slots as scratch (allocator base, segment
/// source), so the stack check is off; the values are observed through the
/// calls they feed. Returns 1 in `al` on success.
///
/// Original: 0x0069BEF0 (thiscall, four stack words, callee pops 16).
lf_checker_rt::export!(thiscall, rw_0069BEF0(this: u32, samples: u32, count: u32, stride: u32, tolbits: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x0C;
        const SEG_OFF: u32 = 0x0E;
        const RANGE_OFF: u32 = 0x10;
        const MIN_OFF: u32 = 0x14;
        const ELEM_SIZE: u32 = 8;
        const HI_SEED_VA: u32 = 0xFE8638;
        const LO_SEED_VA: u32 = 0xFE8D18;
        const TLS_SLOT: usize = 0;
        const HEAPOBJ_OFF: u32 = 8;
        const ALLOC_SLOT: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        const ALLOC_HINT: u32 = 0x10;
        const ALLOC: u32 = 1;
        const SEGM: u32 = 2;
        const FREE: u32 = 3;
        const KEYS: u32 = 4;
        const PURGE: u32 = 5;
        const COPYSEG: u32 = 6;
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
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        if (count as i32) < 3 {
            return 0;
        }
        let tol = f32::from_bits(tolbits);
        if tol == 0.0 {
            return 0;
        }
        let step = stride.wrapping_mul(4).wrapping_add(4);
        // ja skips the store, so each tracker updates unless it is already
        // past the sample (NaN updates, being unordered): the tiny-seeded
        // register tracks the maximum, the huge-seeded one the minimum.
        let mut hi = f32::from_bits(rd32(lf_checker_rt::relocated(HI_SEED_VA)));
        let mut lo = f32::from_bits(rd32(lf_checker_rt::relocated(LO_SEED_VA)));
        let mut p = samples;
        let mut k = count;
        while k != 0 {
            let s = f32::from_bits(rd32(p));
            if !(hi > s) {
                hi = s;
            }
            if !(s > lo) {
                lo = s;
            }
            p = p.wrapping_add(step);
            k -= 1;
        }
        let range = fsub(hi, lo);
        wr32(this + MIN_OFF, lo.to_bits());
        let one = 1.0f32;
        let inv = fdiv(one, range);
        wr32(this + RANGE_OFF, range.to_bits());
        let tls_base = lf_checker_rt::tls_slot(TLS_SLOT);
        let heap_obj = rd32(tls_base + HEAPOBJ_OFF);
        let vtable = rd32(heap_obj);
        let (sz, ov) = count.overflowing_mul(4);
        let size = if ov { 0xFFFFFFFFu32 } else { sz };
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + ALLOC_SLOT) as usize) };
        let buf = alloc(heap_obj, size, ALLOC_HINT, 0);
        let minv = f32::from_bits(rd32(this + MIN_OFF));
        let mut done: u32 = 0;
        if (count as i32) >= 4 {
            let groups = (count.wrapping_sub(4) >> 2).wrapping_add(1);
            let mut g = 0u32;
            let mut sp = samples;
            let mut dp = buf;
            while g < groups {
                let mut j = 0u32;
                while j < 4 {
                    let s = f32::from_bits(rd32(sp));
                    wr32(dp, fmul(fsub(s, minv), inv).to_bits());
                    sp = sp.wrapping_add(step);
                    dp = dp.wrapping_add(4);
                    j += 1;
                }
                g += 1;
            }
            done = groups.wrapping_mul(4);
        }
        while (done as i32) < (count as i32) {
            let s = f32::from_bits(rd32(samples.wrapping_add(done.wrapping_mul(step))));
            wr32(buf.wrapping_add(done.wrapping_mul(4)), fmul(fsub(s, minv), inv).to_bits());
            done += 1;
        }
        let mut s = [0u32, 0u32, 0u32];
        let sp = s.as_mut_ptr() as u32;
        let ans: u32 = lf_checker_rt::callee_thiscall!(SEGM, u32, this, buf, 0,
            count.wrapping_sub(1), tolbits, sp);
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vtable + FREE_SLOT) as usize) };
        if (ans as u8) == 0 {
            if buf != 0 {
                free(heap_obj, buf);
            }
            let _ = lf_checker_rt::callee_thiscall!(PURGE, u32, sp);
            return 0;
        }
        let mut segleft = s[2];
        let block: u32;
        if segleft != 0 {
            block = lf_checker_rt::callee_stdcall!(KEYS, u32, segleft);
            segleft = s[2];
        } else {
            block = 0;
        }
        let mut tail = s[1];
        let mut node = s[0];
        wr32(this + BASE_OFF, block);
        unsafe { ((this + SEG_OFF) as *mut u16).write_unaligned(segleft as u16) };
        while node != 0 {
            let nxt = rd32(node.wrapping_add(4));
            wr32(node.wrapping_add(4), 0);
            let idx = rd16(this + COUNT_OFF);
            if tail == node {
                tail = 0;
            }
            segleft = segleft.wrapping_sub(1);
            s[2] = segleft;
            s[1] = tail;
            let dst = block.wrapping_add(idx.wrapping_mul(ELEM_SIZE));
            wr32(this + COUNT_OFF, idx.wrapping_add(1) & 0xFFFF | (rd32(this + COUNT_OFF) & 0xFFFF0000));
            let src = rd32(node);
            let _ = lf_checker_rt::callee_thiscall!(COPYSEG, u32, dst, src);
            let src2 = rd32(node);
            if src2 != 0 {
                let sub = rd32(src2.wrapping_add(4));
                if sub != 0 {
                    free(heap_obj, sub);
                }
                free(heap_obj, src2);
            }
            free(heap_obj, node);
            node = nxt;
        }
        s[0] = node;
        if buf != 0 {
            free(heap_obj, buf);
        }
        let _ = lf_checker_rt::callee_thiscall!(PURGE, u32, sp);
        1
    }
});
