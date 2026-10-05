// original: 0x0069AF00 rage::crAnimChannelRawQuaternion::vf16

/// Serializer: frees the old buffer, asks the intercepted direct
/// callee to size the member at `+8`, then copies `count`
/// quaternions from the source. After the first, each new entry is
/// sign-aligned with its predecessor: when their dot product is
/// negative the whole new entry is xored with the sign mask kept
/// in the static at file address `0x00FE8FA0`. Float operation order is
/// the original's. Returns the destination base with its low byte
/// set to 1 (1 when nothing was copied).
///
/// Original: 0x0069AF00 (thiscall, source, count, one unused word).
lf_checker_rt::export!(thiscall, rw_0069AF00(this: u32, src: u32, count: u32, _u: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        // Thread-allocator chain: tls slot 0 -> [+8] -> vtable slot +0xc.
        let heap_obj = rd32(lf_checker_rt::tls_slot(0) + 8);
        let vtable = rd32(heap_obj);
        let free_mem: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + 0xC) as usize);
        let old = rd32(this + 8);
        if old != 0 {
            free_mem(heap_obj, old);
        }
        wr32(this + 8, 0);
        wr32(this + 8 + 4, 0);
        lf_checker_rt::callee_thiscall!(3, u32, this + 8, count);
        let mask = lf_checker_rt::global::<u32>(0x00FE8FA0).read_unaligned();
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        let mut last: u32 = 0;
        let mut i: u32 = 0;
        while (i as i32) < (count as i32) {
            let dst = rd32(this + 8);
            last = dst;
            let so = src.wrapping_add(i.wrapping_mul(16));
            let dd = dst.wrapping_add(i.wrapping_mul(16));
            wr32(dd, rd32(so));
            wr32(dd.wrapping_add(4), rd32(so.wrapping_add(4)));
            wr32(dd.wrapping_add(8), rd32(so.wrapping_add(8)));
            wr32(dd.wrapping_add(12), rd32(so.wrapping_add(12)));
            if (i as i32) > 0 {
                let prev = dd.wrapping_sub(16);
                let dot = add(
                    add(add(mul(rdf(prev.wrapping_add(4)), rdf(dd.wrapping_add(4))),
                                mul(rdf(prev), rdf(dd))),
                            mul(rdf(prev.wrapping_add(8)), rdf(dd.wrapping_add(8)))),
                        mul(rdf(prev.wrapping_add(12)), rdf(dd.wrapping_add(12))));
                if dot < 0.0 {
                    wr32(dd, rdf(dd).to_bits() ^ mask);
                    wr32(dd.wrapping_add(4), rdf(dd.wrapping_add(4)).to_bits() ^ mask);
                    wr32(dd.wrapping_add(8), rdf(dd.wrapping_add(8)).to_bits() ^ mask);
                    wr32(dd.wrapping_add(12), rdf(dd.wrapping_add(12)).to_bits() ^ mask);
                }
            }
            i = i.wrapping_add(1);
        }
        (last & 0xFFFF_FF00) | 1

    }
});
