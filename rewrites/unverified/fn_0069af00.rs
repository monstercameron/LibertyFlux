// original: 0x0069AF00 rage::crAnimChannelRawQuaternion::vf16

/// Rebuild a raw-quaternion channel, flipping sign for continuity.
///
/// `thiscall` with the channel in ECX and source, count plus one ignored word
/// on the stack. Frees the old buffer through the TLS allocator, runs the
/// (intercepted) resize for `count` quaternions, then copies 16 bytes per
/// quaternion; from the second one on, dots it against its predecessor in the
/// original's exact SSE order and negates all four lanes (via the sign mask
/// in the read-only global) when the dot is strictly negative. A NaN dot
/// keeps the sign, matching `comiss`. Returns the buffer address with its low
/// byte forced to 1, or the resize answer treated the same way when the count
/// is not positive.
/// Original: 0x0069AF00, 247 bytes.
lf_checker_rt::export!(thiscall, rw_0069AF00(this: u32, src: u32, count: u32, _a2: u32) -> u32 {
    unsafe {
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        const TLS_MGR_OFF: u32 = 0x08;
        const VT_ALLOC_SLOT: u32 = 0x08;
        const VT_FREE_SLOT: u32 = 0x0c;
        const ALLOC_ALIGN: u32 = 0x10;
        let tls = lf_checker_rt::tls_slot(0);
        let mgr = rd32(tls + TLS_MGR_OFF);
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(mgr) + VT_FREE_SLOT) as usize);
        let inner = this.wrapping_add(8);
        let old = rd32(inner);
        if old != 0 {
            let _ = free(mgr, old);
        }
        wr32(inner, 0);
        wr32(inner + 4, 0);
        let ans: u32 = lf_checker_rt::callee_thiscall!(2, u32, inner, count);
        let mask: u32 = (lf_checker_rt::global::<u32>(0x00FE8FA0) as *const u32).read_unaligned();
        let dst = rd32(inner);
        let mut i: i32 = 0;
        let total = count as i32;
        while i < total {
            let s = src + (i as u32) * 16;
            let d = dst + (i as u32) * 16;
            wr32(d, rd32(s));
            wr32(d + 4, rd32(s + 4));
            wr32(d + 8, rd32(s + 8));
            wr32(d + 12, rd32(s + 12));
            if i > 0 {
                let pd = d - 16;
                let px = f32::from_bits(rd32(pd));
                let py = f32::from_bits(rd32(pd + 4));
                let pz = f32::from_bits(rd32(pd + 8));
                let pw = f32::from_bits(rd32(pd + 12));
                let cx = f32::from_bits(rd32(d));
                let cy = f32::from_bits(rd32(d + 4));
                let cz = f32::from_bits(rd32(d + 8));
                let cw = f32::from_bits(rd32(d + 12));
                let tx = mul(px, cx);
                let ty = mul(py, cy);
                let mut dot = add(ty, tx);
                dot = add(dot, mul(pz, cz));
                dot = add(dot, mul(pw, cw));
                if 0.0 > dot {
                    wr32(d, rd32(d) ^ mask);
                    wr32(d + 4, rd32(d + 4) ^ mask);
                    wr32(d + 8, rd32(d + 8) ^ mask);
                    wr32(d + 12, rd32(d + 12) ^ mask);
                }
            }
            i += 1;
        }
        let base = if total > 0 { dst } else { ans };
        (base & 0xFFFF_FF00) | 1
    }
});
