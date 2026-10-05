// original: 0x00de71f0 UITimeOverview::vf88

/// Refresh the time-overview rows and rewrite their time labels.
///
/// `this` is the panel object (thiscall, no stack arguments). The entry
/// counter at `this+0x224` is incremented when nonzero; when it is above 7
/// the rows are refreshed, otherwise only the label block below may run.
/// The refresh asks the panel hook (slot +0x8c, x87 float result) for a
/// value, scales it by the 0.125 factor and walks the eight first-row
/// objects at `this+0x1e0`: three query triples (a build call with two
/// in-place words, a 24-byte descriptor copy passed with a tag to slot
/// +0x114, a release call), a 2.0 step call, a running accumulation of the
/// scaled value, a zero store at +0x1d8, two calls on the sibling object at
/// +0x20 (a 10.0 pair, a flag), a fourth triple feeding slot +0x4c, and a
/// (6, result) call. The end object at `this+0x220` gets three triples, the
/// 2.0 step and the zero store, and the counter resets to 0. When the flag
/// byte at `this+0x228` is set, each row decomposes a remainder that
/// starts at the time minus its scaled part and drops by the scaled part
/// per row (the reload before the subtract discards the within-row
/// narrowing, so only the subtract carries over): scale, truncate and
/// subtract, byte by byte, to hours, minutes and seconds, formatted with
/// "%02d:%02d:%02d" per row through the format callee and handed to slot
/// +0x1e0; the flag is then cleared. The return
/// value is the entry counter on the skip path, the end object when the
/// refresh ran without the label block, and the last label call result
/// otherwise, matching the original's register on every path. The build
/// and release callees receive a scratch word the function never writes
/// (always zero).
///
/// A null row, sibling or end object faults on the unconditional vtable
/// read; the rewrite reads the same way so fault trials match. Float
/// arithmetic is bit-exact in the original's operand order, and truncation
/// follows cvttss2si (out of range or NaN yields 0x80000000, whose low byte
/// is then used). The trailing security-cookie call is invoked as the
/// plain stub with its register argument skipped: the check itself is CRT
/// boilerplate, identical on both sides.
///
/// Original: 0x00de71f0 (thiscall, no stack words, `(an instruction of the original); ret`).
lf_checker_rt::export!(thiscall, rw_00de71f0(this: u32) -> u32 {
    unsafe {
        const ROW_FIRST: u32 = 0x1e0;
        const ROW_COUNT: u32 = 8;
        const FIELD_END: u32 = 0x220;
        const FIELD_STATE: u32 = 0x224;
        const FIELD_FLAG: u32 = 0x228;
        const FIELD_TIME: u32 = 0x22c;
        const ROW_ZERO_OFF: u32 = 0x1d8;
        const SIB_OFF: u32 = 0x20;
        const VT_HOOK: u32 = 0x8c;
        const VT_QUERY: u32 = 0x114;
        const VT_STEP: u32 = 0x94;
        const VT_PAIR: u32 = 0x80;
        const VT_FLAG1: u32 = 0x200;
        const VT_FEED: u32 = 0x4c;
        const VT_SIX: u32 = 0x104;
        const VT_LABEL: u32 = 0x1e0;
        const G_RATE: u32 = 0xfe87a4;
        const G_SEC: u32 = 0xfe8724;
        const G_MIN: u32 = 0xfe8b80;
        const G_CSEC: u32 = 0xfe8bb0;
        const FMT: u32 = 0xefe844;
        const TWO_BITS: u32 = 0x40000000;
        const TEN_BITS: u32 = 0x41200000;
        const ONE_BITS: u32 = 0x3f800000;
        const THREE_BITS: u32 = 0x40400000;
        const NEG_ONE_BITS: u32 = 0xbf800000;
        const DB_BUILD: u32 = 1;
        const DB_RELEASE: u32 = 2;
        const FMT_CALLEE: u32 = 3;
        const COOKIE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Truncate like cvttss2si: NaN or out of i32 range yields 0x80000000.
        #[inline(always)]
        fn cvt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }
        /// One query triple on `obj`: build call, 24-byte descriptor copy
        /// passed with `tag` to slot +0x114, release call. `slot` is the
        /// untouched scratch word both callees receive (always zero).
        #[inline(always)]
        unsafe fn triple(obj: u32, slot: *mut u32, a0: u32, a1: u32, tag: u32) {
            unsafe {
                let vt = rd32(obj);
                let slot_u = slot as u32;
                let desc: u32 =
                    lf_checker_rt::callee_thiscall!(DB_BUILD, u32, slot_u, a0, a1);
                let w0 = rd32(desc);
                let w1 = rd32(desc.wrapping_add(4));
                let w2 = rd32(desc.wrapping_add(8));
                let w3 = rd32(desc.wrapping_add(12));
                let w4 = rd32(desc.wrapping_add(16));
                let w5 = rd32(desc.wrapping_add(20));
                let query: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(VT_QUERY)) as usize);
                let _ = query(obj, tag, w0, w1, w2, w3, w4, w5);
                let _: u32 = lf_checker_rt::callee_thiscall!(DB_RELEASE, u32, slot_u);
            }
        }

        let mut state = rd32(this.wrapping_add(FIELD_STATE));
        if state != 0 {
            state = state.wrapping_add(1);
            wr32(this.wrapping_add(FIELD_STATE), state);
        }
        let mut eax_ret = state;
        let mut end_obj = 0u32;
        let mut big_ran = false;
        if state > 7 {
            let hook: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(rd32(rd32(this).wrapping_add(VT_HOOK)) as usize);
            let k0 = mul(hook(this), rdf(lf_checker_rt::relocated(G_RATE)));
            let mut acc = 0.0f32;
            let mut zeroslot: u32 = 0;
            let mut field = this.wrapping_add(ROW_FIRST);
            let mut row = 0u32;
            while row < ROW_COUNT {
                let obj = rd32(field);
                let slot = &mut zeroslot as *mut u32;
                triple(obj, slot, acc.to_bits(), 0, 2);
                triple(obj, slot, ONE_BITS, 0, 4);
                triple(obj, slot, 0, 0, 0x10);
                let vt = rd32(obj);
                let step: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(VT_STEP)) as usize);
                let _ = step(obj, TWO_BITS);
                acc = add(acc, k0);
                wr32(obj.wrapping_add(ROW_ZERO_OFF), 0);
                let sib = rd32(field.wrapping_add(SIB_OFF));
                let sibvt = rd32(sib);
                let pair: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(sibvt.wrapping_add(VT_PAIR)) as usize);
                let _ = pair(sib, TEN_BITS, TEN_BITS);
                let flag1: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(sibvt.wrapping_add(VT_FLAG1)) as usize);
                let _ = flag1(sib, 1);
                let desc4: u32 = lf_checker_rt::callee_thiscall!(
                    DB_BUILD, u32, slot as u32, THREE_BITS, NEG_ONE_BITS
                );
                let u0 = rd32(desc4);
                let u1 = rd32(desc4.wrapping_add(4));
                let u2 = rd32(desc4.wrapping_add(8));
                let u3 = rd32(desc4.wrapping_add(12));
                let u4 = rd32(desc4.wrapping_add(16));
                let u5 = rd32(desc4.wrapping_add(20));
                let feed: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(VT_FEED)) as usize);
                let fed = feed(obj, 0x0c, u0, u1, u2, u3, u4, u5);
                let six: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(sibvt.wrapping_add(VT_SIX)) as usize);
                let _ = six(sib, 6, fed);
                let _: u32 = lf_checker_rt::callee_thiscall!(DB_RELEASE, u32, slot as u32);
                row += 1;
                field = field.wrapping_add(4);
            }
            let end = rd32(this.wrapping_add(FIELD_END));
            end_obj = end;
            let slot = &mut zeroslot as *mut u32;
            triple(end, slot, 0, 0, 8);
            triple(end, slot, 0, ONE_BITS, 4);
            triple(end, slot, 0, 0, 0x10);
            let endvt = rd32(end);
            let estep: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(endvt.wrapping_add(VT_STEP)) as usize);
            // The step result is discarded: the original reloads the end
            // object into eax right after, which is the return value when
            // the label block below does not run.
            let _ = estep(end, TWO_BITS);
            wr32(end.wrapping_add(ROW_ZERO_OFF), 0);
            wr32(this.wrapping_add(FIELD_STATE), 0);
            big_ran = true;
        }
        if rd8(this.wrapping_add(FIELD_FLAG)) != 0 {
            let t = rdf(this.wrapping_add(FIELD_TIME));
            let k0b = mul(t, rdf(lf_checker_rt::relocated(G_RATE)));
            let mut rem = sub(t, k0b);
            let mut field = this.wrapping_add(FIELD_END).wrapping_sub(4);
            let mut n = 7i32;
            loop {
                // Scratch decomposition of this row's remainder: the
                // reload before the per-row subtract discards these
                // within-row updates, so only the subtract carries over.
                let mut r = rem;
                let edx = cvt(mul(r, rdf(lf_checker_rt::relocated(G_SEC)))) as u8 as u32;
                r = sub(r, mul(edx as f32, rdf(lf_checker_rt::relocated(G_MIN))));
                let ecx = cvt(r) as u8 as u32;
                r = sub(r, ecx as f32);
                r = mul(r, rdf(lf_checker_rt::relocated(G_CSEC)));
                let e = cvt(r) as u8 as u32;
                let mut buf: u32 = 0;
                let bufptr = (&mut buf as *mut u32) as u32;
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    FMT_CALLEE, u32, bufptr, lf_checker_rt::relocated(FMT), edx, ecx, e
                );
                let tobj = rd32(field);
                let tovt = rd32(tobj);
                let label: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(tovt.wrapping_add(VT_LABEL)) as usize);
                eax_ret = label(tobj, bufptr, 0);
                rem = sub(rem, k0b);
                field = field.wrapping_sub(4);
                if n == 0 {
                    break;
                }
                n -= 1;
            }
            wr8(this.wrapping_add(FIELD_FLAG), 0);
        } else if big_ran {
            eax_ret = end_obj;
        }
        let _: u32 = lf_checker_rt::callee_fastcall!(COOKIE, u32, 0u32, 0u32);
        eax_ret
    }
});
