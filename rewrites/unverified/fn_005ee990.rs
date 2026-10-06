// original: 0x005ee990 CHtmlTextFormat::~CHtmlTextFormat_2 (merged symbol, low confidence)

/// Release a text format's cached rows (merged name kept; behaves as teardown).
///
/// `this` is the format object. A global enable byte gates everything, and a
/// global row count bounds the loop (zero count skips everything past the
/// head loads, signed less against the non-negative loop index); both zero
/// paths skip to the tail call. A chain walk from the global array's first
/// slot follows next links while the kind word is exactly `0x12` (a null
/// link faults reading through it, kept as fault parity); callee 1
/// (thiscall) fills four frame words, of which only the third is read: it is
/// divided by `[this+0x18]` and passed with zero to callee 2 (cdecl). The
/// negated `[this+0x14]`, the context floats and most frame initialisation
/// are never read back; one frame word keeps a relocated address which the
/// divisor call observes.
///
/// Each row loads its pointer from the global array and resolves two nodes
/// through callee 3 (two sites, sequenced answers): two null nodes end the
/// iteration; a live second node runs a prepare pair (callees 4 and 5) and
/// the divisor call (callee 8, cdecl, four words); otherwise the first
/// node's kind word dispatches (exact integer compares): `0x26` runs a
/// fetch (callee 6, thiscall, out-word) and convert (callee 7, thiscall)
/// pair feeding the same divisor call, `0x5` runs an x87 measure whose
/// result, scaled by `[this+0x1c]`, reaches frame method A in xmm2 with the
/// constant 1.0 in xmm1, then frame method B, `0x15`/`0xc` run frame method
/// A with two vector loads, anything else ends the iteration. After the loop
/// frame method B runs, then the tail call whose answer is returned, then
/// the preserving cookie check.
///
/// Original: 0x005ee990 (thiscall, ECX = this, no stack words).
lf_checker_rt::export!(thiscall, rw_005ee990(this: u32) -> u32 {
    unsafe {
        const G_ENABLE: u32 = 0x01b4bbe2;
        const G_COUNT: u32 = 0x019e866c;
        const G_ARRAY: u32 = 0x019e8668;
        const CODE_ADDR: u32 = 0x00fe0b40;
        const R_A: u32 = 0x0114e798;
        const R_B: u32 = 0x0114e780;
        const R_C: u32 = 0x0114e7fc;
        const R_CTX: u32 = 0x0116bff0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        // The tail call id, shared by all paths.
        let tail = || -> u32 { lf_checker_rt::callee_cdecl!(12, u32,) };
        if (lf_checker_rt::relocated(G_ENABLE) as *const u8).read() == 0 {
            let r = tail();
            let _ck: u32 = lf_checker_rt::callee_cdecl!(13, u32,);
            return r;
        }
        // Double dereference; values feed dead slots but must be mapped.
        let inner = rd32(rd32(this.wrapping_add(0x10)));
        let _x3 = rdf(inner.wrapping_add(0x18));
        let _x4 = rdf(inner.wrapping_add(0x1c));
        let count = (lf_checker_rt::relocated(G_COUNT) as *const u16).read_unaligned() as u32;
        if count == 0 {
            let r = tail();
            let _ck: u32 = lf_checker_rt::callee_cdecl!(13, u32,);
            return r;
        }
        // Chain walk: start at the first slot's next link, advance while the
        // kind word is exactly 0x12 (equality; no signedness question).
        let node0 = rd32(g32(G_ARRAY));
        let mut esi = rd32(node0.wrapping_add(8));
        if esi != 0 {
            while rd32(esi.wrapping_add(0x14)) == 0x12 {
                esi = rd32(esi.wrapping_add(8));
                if esi == 0 {
                    break;
                }
            }
        }
        let mut w8 = 0u32;
        let mut wc = 0u32;
        let mut w14 = 0u32;
        let mut w18 = 0u32;
        lf_checker_rt::callee_thiscall!(
            1, u32, esi,
            &mut w8 as *mut u32 as u32,
            &mut wc as *mut u32 as u32,
            &mut w14 as *mut u32 as u32,
            &mut w18 as *mut u32 as u32
        );
        // Fault parity: the original reads [esi+0x4c] (null when the chain
        // link was null) before using the fetched word.
        let _nodeword = rd32(esi.wrapping_add(0x4c));
        let _ = _nodeword;
        let w14f = f32::from_bits(w14);
        let q = div(w14f, rdf(this.wrapping_add(0x18)));
        lf_checker_rt::callee_cdecl!(2, u32, 0, q.to_bits());

        // Frame word: relocated address until a 0x26 row zeroes it.
        let mut s1c = lf_checker_rt::relocated(CODE_ADDR);
        let arr = g32(G_ARRAY);
        let mut i = 0u32;
        while (i as i32) < (count as i32) {
            let edi_i = rd32(arr.wrapping_add(i.wrapping_mul(4)));
            let e1: u32 = lf_checker_rt::callee_cdecl!(
                3, u32, edi_i, 0,
                lf_checker_rt::relocated(R_B),
                lf_checker_rt::relocated(R_A),
                0
            );
            let e2: u32 = lf_checker_rt::callee_cdecl!(
                3, u32, edi_i, 0,
                lf_checker_rt::relocated(R_B),
                lf_checker_rt::relocated(R_C),
                0
            );
            if e2 != 0 {
                lf_checker_rt::callee_thiscall!(4, u32, e2.wrapping_add(0x14),);
                lf_checker_rt::callee_cdecl!(5, u32, 1);
                let x0 = 0.0f32;
                let d = div(x0, rdf(this.wrapping_add(0x18)));
                lf_checker_rt::callee_cdecl!(
                    8, u32, d.to_bits(), 0,
                    rd32(e2.wrapping_add(0xdc)),
                    &mut s1c as *mut u32 as u32
                );
            } else if e1 != 0 {
                let kind = rd32(e1.wrapping_add(0xd8));
                if kind == 0x26 {
                    lf_checker_rt::callee_thiscall!(4, u32, e1.wrapping_add(0x14),);
                    lf_checker_rt::callee_cdecl!(5, u32, 1);
                    s1c = 0;
                    let a7: u32 = lf_checker_rt::callee_thiscall!(
                        6, u32, e1,
                        &mut s1c as *mut u32 as u32
                    );
                    let a8: u32 = lf_checker_rt::callee_thiscall!(
                        7, u32,
                        lf_checker_rt::relocated(R_CTX),
                        a7
                    );
                    let x0 = f32::from_bits(s1c);
                    let d = div(x0, rdf(this.wrapping_add(0x18)));
                    lf_checker_rt::callee_cdecl!(
                        8, u32, d.to_bits(), 0, a8, a7
                    );
                } else if kind == 5 {
                    let ans: f32 = lf_checker_rt::callee_cdecl!(9, f32,);
                    // x87 measure stored to the frame slot, scaled by
                    // [this+0x1c]; both vector lanes reach frame method A.
                    let s18 = ans;
                    let x2 = mul(s18, rdf(this.wrapping_add(0x1c)));
                    let x1 = 1.0f32;
                    lf_checker_rt::callee_thiscall!(
                        10, u32,
                        &mut s1c as *mut u32 as u32,
                        x1.to_bits(), x2.to_bits(),
                    );
                    lf_checker_rt::callee_thiscall!(
                        11, u32,
                        &mut s1c as *mut u32 as u32,
                    );
                } else if kind == 0x15 || kind == 0x0c {
                    // Two vector loads that reach frame method A as xmm1/xmm2.
                    let x2 = rdf(e1.wrapping_add(0x24));
                    let x1 = rdf(e1.wrapping_add(0x20));
                    lf_checker_rt::callee_thiscall!(
                        10, u32,
                        &mut s1c as *mut u32 as u32,
                        x1.to_bits(), x2.to_bits(),
                    );
                }
            }
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(
            11, u32,
            &mut s1c as *mut u32 as u32,
        );
        let r = tail();
        let _ck: u32 = lf_checker_rt::callee_cdecl!(13, u32,);
        r
    }
});
