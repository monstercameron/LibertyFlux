// original: 0x00e46a00 MO_OFF (symbols)

/// Register a table of paired text entries into a large object, then append
/// entries derived from game state.
///
/// `this` points to an object of at least 0x1e2d4 bytes. The function keeps a
/// counter at `+0x1e2d0` (starts at 0) and treats the object as two parallel
/// arrays of records `REC` (0x804) bytes long: name slots at `+0x1e0` and
/// value slots at `+0x5e0`. Each entry copies a looked-up or formatted string
/// into the current name slot (callee 2) and usually a second string into the
/// matching value slot, then sets flag bytes at record offsets 0x800 (name
/// done), 0x801 (value done), 0x802 (pair done, simple entries) or 0x803
/// (final entry) and advances the counter by 1 or 2.
///
/// The first 19 entries are unconditional: plain lookups (callee 1, thiscall
/// on a shared registry object with one constant argument) whose results feed
/// the string copier (callee 2, cdecl of three words), four float-formatted
/// entries (single-precision divide then, except the first, multiply by a
/// constant; converted to double and passed to the formatter, callee 5, as
/// two words), one six-word formatted entry whose extra arguments are three
/// words produced by callee 8, two integer-formatted entries (callee 6) and
/// several mode lookups (callees 9, 10, 11) with two single-word branches on
/// global flags. Three conditional sections follow, each guarded by a global
/// pointer (re-read after a registry critical section when null): a three-way
/// branch on two global bytes selects a direct copy, a converted copy through
/// callees 15/16, or a normalised copy through callees 15/19/20/3. A counted
/// loop over a global begin/end pointer pair appends one name entry per live
/// element (same three-way choice, value side omitted), then two more guarded
/// sections append name entries from a second pointer pair and its linked
/// word. The tail registers one last lookup pair and one entry whose source
/// is fixed bytes inside this object, then returns counter * REC.
///
/// Callees are intercepted by the checker and identified by contract id; the
/// numbers above are those ids. The security-cookie check at the end is a
/// call like any other (id 24, preserves registers). Float order is the
/// original's: (a / b) [* k], operands pinned with `black_box`.
///
/// Original: 0x00e46a00 (thiscall, no stack arguments, returns counter * REC
/// in eax, callee-saved registers preserved).
lf_checker_rt::export!(thiscall, rw_00e46a00(this: u32) -> u32 {
    unsafe {
        const REC: u32 = 0x804;
        const A_BASE: u32 = 0x1e0;
        const B_BASE: u32 = 0x5e0;
        const CTR: u32 = 0x1e2d0;
        const FLAG_A: u32 = 0x9e0;
        const FLAG_B: u32 = 0x9e1;
        const FLAG_PAIR: u32 = 0x9e2;
        const FLAG_LAST: u32 = 0x9e3;
        const TAIL_SRC_B: u32 = 0x1e6d6;
        const TAIL_SRC_A: u32 = 0x1e2d6;
        const REGISTRY: u32 = 0x116bff0;
        const G_E58: u32 = 0x19d2e58;
        const G_E5C: u32 = 0x19d2e5c;
        const G_E64: u32 = 0x19d2e64;
        const G_E68: u32 = 0x19d2e68;
        const G_E70: u32 = 0x19d2e70;
        const G_E9C: u32 = 0x19d2e9c;
        const G_E94: u32 = 0x1160e94;
        const G_E98: u32 = 0x1160e98;
        const G_EA0: u32 = 0x1160ea0;
        const G_EAC: u32 = 0x1160eac;
        const G_EB0: u32 = 0x1160eb0;
        const G_L250: u32 = 0x116c250;
        const G_L253: u32 = 0x116c253;
        const G_FB8: u32 = 0x19d2eb8;
        const G_FBC: u32 = 0x19d2ebc;
        const G_FC0: u32 = 0x19d2ec0;
        const G_FC4: u32 = 0x19d2ec4;
        const G_FD0: u32 = 0x19d2ed0;
        const F_MULC: u32 = 0xfe8bb0;
        const MODE_A: u8 = 0x6a;
        const MODE_B: u8 = 0x72;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gu32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gu8(va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn rva(va: u32) -> u32 {
            lf_checker_rt::relocated(va)
        }
        #[inline(always)]
        fn slot(this: u32, base: u32, n: u32) -> u32 {
            this.wrapping_add(base).wrapping_add(n.wrapping_mul(REC))
        }
        #[inline(always)]
        fn flag_at(this: u32, flag: u32, n: u32) -> u32 {
            this.wrapping_add(flag).wrapping_add(n.wrapping_mul(REC))
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        // Registry lookup: callee 1 (thiscall on the shared object, 1 word).
        #[inline(always)]
        unsafe fn lookup(key: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_thiscall!(1, u32, rva(REGISTRY), rva(key))
            }
        }
        // String copy into a slot: callee 2 (dst, src, count).
        #[inline(always)]
        unsafe fn stra(dst: u32, src: u32, count: u32) {
            unsafe {
                lf_checker_rt::callee_cdecl!(2, u32, dst, src, count);
            }
        }
        // String copy from a scratch buffer: callee 3 (dst, src, count).
        #[inline(always)]
        unsafe fn strb(dst: u32, src: u32) {
            unsafe {
                lf_checker_rt::callee_cdecl!(3, u32, dst, src, 0xffff_ffff);
            }
        }
        // Registry critical section used when a global pointer is null.
        #[inline(always)]
        unsafe fn triple(lock: u32) {
            unsafe {
                let tok = lf_checker_rt::callee_thiscall!(12, u32, lock, rva(G_E58));
                lf_checker_rt::callee_thiscall!(13, u32, tok);
                lf_checker_rt::callee_thiscall!(14, u32, lock);
            }
        }

        let mut buf_fmt = [0u32; 8];
        let mut buf_str = [0u32; 10];
        let mut buf_aux = [0u32; 8];
        let mut lock = [0u32; 8];
        let mut b0 = [0u32; 1];
        let mut b1 = [0u32; 1];
        let mut b2 = [0u32; 1];
        let mut b3 = [0u8; 8];
        let fmt = buf_fmt.as_mut_ptr() as u32;
        let s = buf_str.as_mut_ptr() as u32;
        let s2 = buf_aux.as_mut_ptr() as u32;
        let lockp = lock.as_mut_ptr() as u32;

        wr32(this.wrapping_add(CTR), 0);
        let mut n: u32 = 0;
        // Block 1: plain pair.
        let e = lookup(0xf163cc);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        wr8(flag_at(this, FLAG_PAIR, n), 1);
        n = n.wrapping_add(2);
        wr32(this.wrapping_add(CTR), n);
        // Block 2: plain name + float value (divide only).
        let e = lookup(0xf163d8);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let q = fdiv(
            f32::from_bits(gu32(G_FD0)),
            f32::from_bits(gu32(G_FC4)),
        );
        let d = (q as f64).to_bits();
        lf_checker_rt::callee_cdecl!(5, u32, fmt, 0x200, rva(0xf163e4), d as u32, (d >> 32) as u32);
        strb(slot(this, B_BASE, n), fmt);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 3: plain name + float value.
        let e = lookup(0xf163f0);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let q = fdiv(
            f32::from_bits(gu32(G_FB8)),
            f32::from_bits(gu32(G_FD0)),
        );
        let m = fmul(q, f32::from_bits(gu32(F_MULC)));
        let d = (m as f64).to_bits();
        lf_checker_rt::callee_cdecl!(5, u32, fmt, 0x200, rva(0xf163f8), d as u32, (d >> 32) as u32);
        strb(slot(this, B_BASE, n), fmt);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 4: plain name + float value.
        let e = lookup(0xf16408);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let q = fdiv(
            f32::from_bits(gu32(G_FBC)),
            f32::from_bits(gu32(G_FD0)),
        );
        let m = fmul(q, f32::from_bits(gu32(F_MULC)));
        let d = (m as f64).to_bits();
        lf_checker_rt::callee_cdecl!(5, u32, fmt, 0x200, rva(0xf16414), d as u32, (d >> 32) as u32);
        strb(slot(this, B_BASE, n), fmt);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 5: plain name + float value.
        let e = lookup(0xf16424);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let q = fdiv(
            f32::from_bits(gu32(G_FC0)),
            f32::from_bits(gu32(G_FD0)),
        );
        let m = fmul(q, f32::from_bits(gu32(F_MULC)));
        let d = (m as f64).to_bits();
        lf_checker_rt::callee_cdecl!(5, u32, fmt, 0x200, rva(0xf16434), d as u32, (d >> 32) as u32);
        strb(slot(this, B_BASE, n), fmt);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(2);
        wr32(this.wrapping_add(CTR), n);
        // Block 6: plain pair.
        let e = lookup(0xf16444);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        wr8(flag_at(this, FLAG_PAIR, n), 1);
        n = n.wrapping_add(2);
        wr32(this.wrapping_add(CTR), n);
        // Block 7: plain name + six-word formatted value.
        let e = lookup(0xf16454);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        wr8(b3.as_mut_ptr() as u32, 0);
        lf_checker_rt::callee_cdecl!(
            8, u32,
            b0.as_mut_ptr() as u32,
            b1.as_mut_ptr() as u32,
            b2.as_mut_ptr() as u32,
            b3.as_mut_ptr() as u32
        );
        let r0 = b0[0];
        let r1 = b1[0];
        let r2 = b2[0];
        lf_checker_rt::callee_cdecl!(7, u32, fmt, 0x200, rva(0xf16464), r0, r1, r2);
        strb(slot(this, B_BASE, n), fmt);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 8: plain name + mode value.
        let e = lookup(0xf16488);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = lf_checker_rt::callee_cdecl!(9, u32, 0x91, 0);
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 9: name + mode-or-lookup value.
        let e = lookup(0xf16490);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = if gu32(G_EB0) != 0 {
            lf_checker_rt::callee_cdecl!(9, u32, 0x9a, 0xffff_ffff)
        } else {
            lookup(0xf1649c)
        };
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 10: name + mode value.
        let e = lookup(0xf164a4);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = lf_checker_rt::callee_cdecl!(9, u32, 0x9b, 0);
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 11: name + mode-or-lookup value.
        let e = lookup(0xf164b4);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = if gu32(G_EAC) != 0 {
            lf_checker_rt::callee_cdecl!(9, u32, 0x99, 0xffff_ffff)
        } else {
            lookup(0xf164bc)
        };
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 12: name + second-mode value.
        let e = lookup(0xf164c4);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = lf_checker_rt::callee_cdecl!(10, u32, 0x92, 0);
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 13: name + third-mode value.
        let e = lookup(0xf164d0);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = lf_checker_rt::callee_cdecl!(11, u32, 0x98, 0);
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 14: name + integer-formatted value.
        let e = lookup(0xf164e0);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = gu32(G_E94).wrapping_add(1);
        lf_checker_rt::callee_cdecl!(6, u32, fmt, 0x200, rva(0xf164ec), v);
        strb(slot(this, B_BASE, n), fmt);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 15: name + integer-formatted value.
        let e = lookup(0xf164f4);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = gu32(G_E98).wrapping_add(1);
        lf_checker_rt::callee_cdecl!(6, u32, fmt, 0x200, rva(0xf16500), v);
        strb(slot(this, B_BASE, n), fmt);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 16: name + one of two lookups.
        let e = lookup(0xf16508);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = if gu32(G_EA0) != 0 {
            lookup(0xf16510)
        } else {
            lookup(0xf16518)
        };
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 17: name + lookup value, double step.
        let e = lookup(0xf16520);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let v = lookup(0xf1652c);
        stra(slot(this, B_BASE, n), v, 0xffff_ffff);
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(2);
        wr32(this.wrapping_add(CTR), n);
        // Block 18: plain pair, double step.
        let e = lookup(0xf16534);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        wr8(flag_at(this, FLAG_PAIR, n), 1);
        n = n.wrapping_add(2);
        wr32(this.wrapping_add(CTR), n);
        // Block 19: name + guarded value section 1.
        let e = lookup(0xf16540);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let mut ebx = gu32(G_E70);
        if ebx == 0 {
            triple(lockp);
            ebx = gu32(G_E70);
        }
        if ebx == 0 {
            lf_checker_rt::callee_cdecl!(18, u32, rva(0xf1654c), slot(this, B_BASE, n));
        } else {
            let al = gu8(G_L250);
            if al == MODE_A {
                wr16(s, 0);
                lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                lf_checker_rt::callee_cdecl!(19, u32, ebx, s);
                lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                strb(slot(this, B_BASE, n), s);
            } else {
                let cl = gu8(G_L253);
                if cl != 0 {
                    wr16(s, 0);
                    lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                    lf_checker_rt::callee_cdecl!(19, u32, ebx, s);
                    lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                    strb(slot(this, B_BASE, n), s);
                } else if al == MODE_B {
                    wr8(s, cl);
                    lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(1), 0, 0xff);
                    lf_checker_rt::callee_thiscall!(16, u32, this, ebx, s, 0x100);
                    lf_checker_rt::callee_cdecl!(18, u32, s, slot(this, B_BASE, n));
                } else {
                    lf_checker_rt::callee_cdecl!(18, u32, ebx, slot(this, B_BASE, n));
                }
            }
        }
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Block 20: name + guarded value section 2.
        let e = lookup(0xf16550);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        let ebx = gu32(G_E9C);
        if ebx == 0 {
            triple(lockp);
            lf_checker_rt::callee_cdecl!(18, u32, rva(0xf1655c), slot(this, B_BASE, n));
        } else {
            let al = gu8(G_L250);
            if al == MODE_A {
                wr16(s, 0);
                lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                lf_checker_rt::callee_cdecl!(4, u32, s, ebx, 0xfe);
                lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                strb(slot(this, B_BASE, n), s);
            } else {
                let cl = gu8(G_L253);
                if cl != 0 {
                    wr16(s, 0);
                    lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                    lf_checker_rt::callee_cdecl!(4, u32, s, ebx, 0xfe);
                    lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                    strb(slot(this, B_BASE, n), s);
                } else if al == MODE_B {
                    wr8(s, cl);
                    lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(1), 0, 0xff);
                    wr16(s2, 0);
                    lf_checker_rt::callee_cdecl!(15, u32, s2.wrapping_add(2), 0, 0x1fe);
                    lf_checker_rt::callee_cdecl!(21, u32, ebx, s2, 0x100);
                    lf_checker_rt::callee_cdecl!(22, u32, s2, s);
                    lf_checker_rt::callee_cdecl!(18, u32, s, slot(this, B_BASE, n));
                } else {
                    stra(slot(this, B_BASE, n), ebx, 0xffff_ffff);
                }
            }
        }
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Counted loop over the global pointer pair: one name entry per live
        // element, then one extra counter step.
        let mut eax = gu32(G_E68);
        let mut ecx = gu32(G_E64);
        if eax == ecx {
            triple(lockp);
            eax = gu32(G_E68);
            ecx = gu32(G_E64);
        }
        let mut rem = (eax.wrapping_sub(ecx) as i32 >> 3) as u32;
        let mut ebp = if rem != 0 { ecx } else { 0 };
        if rem != 0 {
            while rem != 0 {
                if ebp != 0 {
                    let ebx = rd32(ebp);
                    if ebx != 0 {
                        let al = gu8(G_L250);
                        if al == MODE_A {
                            wr16(s, 0);
                            lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                            lf_checker_rt::callee_cdecl!(4, u32, s, ebx, 0xff);
                            lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                            strb(slot(this, A_BASE, n), s);
                        } else {
                            let cl = gu8(G_L253);
                            if cl != 0 {
                                wr16(s, 0);
                                lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                                lf_checker_rt::callee_cdecl!(4, u32, s, ebx, 0xff);
                                lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                                strb(slot(this, A_BASE, n), s);
                            } else if al == MODE_B {
                                wr8(s, cl);
                                lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(1), 0, 0xff);
                                lf_checker_rt::callee_cdecl!(23, u32, ebx, s);
                                lf_checker_rt::callee_thiscall!(17, u32, this, s, s, 0x100);
                                lf_checker_rt::callee_cdecl!(18, u32, s, slot(this, A_BASE, n));
                            } else {
                                stra(slot(this, A_BASE, n), ebx, 0xffff_ffff);
                            }
                        }
                        wr8(flag_at(this, FLAG_A, n), 1);
                        n = n.wrapping_add(1);
                        wr32(this.wrapping_add(CTR), n);
                        ebp = ebp.wrapping_add(8);
                    }
                }
                rem = rem.wrapping_sub(1);
            }
        }
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Guarded name section 3 from the second pointer pair; ebx survives
        // for section 4.
        let mut ebx = gu32(G_E58);
        let mut eax = gu32(G_E5C);
        if ebx == eax {
            triple(lockp);
            eax = gu32(G_E5C);
            ebx = gu32(G_E58);
        }
        let len = eax.wrapping_sub(ebx);
        if len & 0xffff_fff8 == 0 {
            ebx = 0;
        }
        if ebx != 0 {
            let ebp = rd32(ebx);
            if ebp != 0 {
                let al = gu8(G_L250);
                if al == MODE_A {
                    wr16(s, 0);
                    lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                    lf_checker_rt::callee_cdecl!(4, u32, s, ebp, 0xff);
                    lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                    strb(slot(this, A_BASE, n), s);
                } else {
                    let cl = gu8(G_L253);
                    if cl != 0 {
                        wr16(s, 0);
                        lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                        lf_checker_rt::callee_cdecl!(4, u32, s, ebp, 0xff);
                        lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                        strb(slot(this, A_BASE, n), s);
                    } else if al == MODE_B {
                        wr8(s, cl);
                        lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(1), 0, 0xff);
                        wr16(s2, 0);
                        lf_checker_rt::callee_cdecl!(15, u32, s2.wrapping_add(2), 0, 0x1fe);
                        lf_checker_rt::callee_cdecl!(21, u32, ebp, s2, 0x100);
                        lf_checker_rt::callee_cdecl!(22, u32, s2, s);
                        lf_checker_rt::callee_cdecl!(18, u32, s, slot(this, A_BASE, n));
                    } else {
                        stra(slot(this, A_BASE, n), ebp, 0xffff_ffff);
                    }
                }
            } else {
                lf_checker_rt::callee_cdecl!(18, u32, rva(0xf16560), slot(this, A_BASE, n));
            }
        } else {
            lf_checker_rt::callee_cdecl!(18, u32, rva(0xf16560), slot(this, A_BASE, n));
        }
        wr8(flag_at(this, FLAG_A, n), 1);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        // Guarded name section 4 from the linked word.
        if ebx != 0 {
            let linked = rd32(ebx.wrapping_add(4));
            if linked != 0 {
                let al = gu8(G_L250);
                if al == MODE_A {
                    wr16(s, 0);
                    lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                    lf_checker_rt::callee_cdecl!(4, u32, s, linked, 0xff);
                    lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                    strb(slot(this, A_BASE, n), s);
                } else {
                    let dl = gu8(G_L253);
                    if dl != 0 {
                        wr16(s, 0);
                        lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(2), 0, 0x1fe);
                        lf_checker_rt::callee_cdecl!(4, u32, s, linked, 0xff);
                        lf_checker_rt::callee_cdecl!(20, u32, s, 0x100);
                        strb(slot(this, A_BASE, n), s);
                    } else if al == MODE_B {
                        wr8(s, dl);
                        lf_checker_rt::callee_cdecl!(15, u32, s.wrapping_add(1), 0, 0xff);
                        lf_checker_rt::callee_cdecl!(23, u32, linked, s);
                        lf_checker_rt::callee_thiscall!(17, u32, this, s, s, 0x100);
                        lf_checker_rt::callee_cdecl!(18, u32, s, slot(this, A_BASE, n));
                    } else {
                        stra(slot(this, A_BASE, n), linked, 0xffff_ffff);
                    }
                }
            } else {
                lf_checker_rt::callee_cdecl!(18, u32, rva(0xf16564), slot(this, A_BASE, n));
            }
        } else {
            lf_checker_rt::callee_cdecl!(18, u32, rva(0xf16564), slot(this, A_BASE, n));
        }
        wr8(flag_at(this, FLAG_A, n), 1);
        n = n.wrapping_add(2);
        wr32(this.wrapping_add(CTR), n);
        // Tail: last lookup pair, fixed-source pair, cookie check.
        let e = lookup(0xf16568);
        stra(slot(this, A_BASE, n), e, 0xffff_ffff);
        wr8(flag_at(this, FLAG_A, n), 1);
        lf_checker_rt::callee_cdecl!(18, u32, this.wrapping_add(TAIL_SRC_B), slot(this, B_BASE, n));
        wr8(flag_at(this, FLAG_B, n), 1);
        n = n.wrapping_add(2);
        wr32(this.wrapping_add(CTR), n);
        stra(
            slot(this, A_BASE, n),
            this.wrapping_add(TAIL_SRC_A),
            0xffff_ffff,
        );
        wr8(flag_at(this, FLAG_A, n), 1);
        wr8(flag_at(this, FLAG_LAST, n), 1);
        // The original multiplies the counter into eax BEFORE its final
        // increment; the returned value is the pre-increment count.
        let ret = n.wrapping_mul(REC);
        n = n.wrapping_add(1);
        wr32(this.wrapping_add(CTR), n);
        lf_checker_rt::callee_cdecl!(24, u32,);
        ret
    }
});
