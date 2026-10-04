// original: 0x00c661c0 CCutsceneObject::vf51

/// Per-frame update of a cutscene object: state dispatch plus either a
/// two-bone blend (J) or a four-bone average (K) fed to a pose callee.
///
/// `this` is the cutscene object (thiscall, no stack args). Entry calls
/// callee 1 always and callee 2 unless a global byte is set, then two
/// virtual calls on the object's own table (slots `+0x24`/`+0x28`, answered
/// with scripted low bytes): a nonzero first answer runs the C region (a
/// third virtual call plus one of two early returns through callees 6 or
/// 7/8, picked by a flag byte in a global pointer table), while a zero
/// first answer and a zero second answer return the second answer.
/// Otherwise the main region runs: an id selects a table entry (`ebx`) and
/// a matrix pointer (`edi`), a maze over global counters and a scripted
/// word either issues two setup calls (callees 9 and 10, then a global
/// float is stored through a double-indirect heap pointer) or skips them,
/// and a mode word picks J (`[ebx+0x6c] == 1`) or K.
///
/// J blends two bone triples (callee 11 answers the bone set; two scripted
/// indices pick rows) with global weights, folds in a global accumulator
/// triple (zeroed with its flag bit set when the flag was clear, reloaded
/// otherwise), mixes the object's matrix rows, and calls callee 12 with the
/// matrix pointer, three 4-word blocks and five scalar words (two built
/// from a row length and global scales). K averages four bone triples,
/// forms difference triples, blends the same matrix rows, and calls callee
/// 13 the same way with roots of sums of squares. One never-written frame
/// slot is copied into two block tails; the contract defines unread stack
/// as zero, so the rewrite uses 0.0 there. Returns the last callee answer,
/// or a loaded word on the guard exits.
///
/// Float operation order is the original's, pinned through `black_box`.
lf_checker_rt::export!(thiscall, rw_00c661c0(this: u32) -> u32 {
    unsafe {
        const VTABLE_OFF: u32 = 0x0;
        const MATRIX_PTR: u32 = 0x20;
        const SCRIPT_WORD: u32 = 0x2c;
        const TABLE_INDEX: u32 = 0x2e;
        const STORE_CHAIN: u32 = 0x34;
        const OBJ2_PTR: u32 = 0x290;
        const CALL6_ARG1: u32 = 0x294;
        const CALL6_ARG2: u32 = 0x310;
        const ENTRY_SEQ_G: u32 = 0x0159_3310;
        const ENTRY_SEQ_BYTE: u32 = 2;
        const TABLE_G: u32 = 0x0129_5cd8;
        const FLAG_OFF: u32 = 0x8c;
        const MODE_OFF: u32 = 0x6c;
        const PARAM_OFF: u32 = 0xcc;
        const WEIGHT_OFF: u32 = 0x74;
        const SEL_G: u32 = 0x0129_5854;
        const EDX_A_G: u32 = 0x0129_5848;
        const EAX_G: u32 = 0x0129_5858;
        const ECX_G: u32 = 0x0129_584c;
        const WIN_LO_G: u32 = 0x012d_dea0;
        const WIN_HI_G: u32 = 0x012d_deac;
        const WIN_SCALE_G: u32 = 0x00e9_d0d8;
        const SETUP_FLAG_G: u32 = 0x0129_577c;
        const STORE_VAL_G: u32 = 0x012e_22a8;
        const STORE_OFF: u32 = 0x90;
        const ACC_FLAG_G: u32 = 0x016d_ceb0;
        const ACC0_G: u32 = 0x016d_cea0;
        const ACC1_G: u32 = 0x016d_cea4;
        const ACC2_G: u32 = 0x016d_cea8;
        const K1_G: u32 = 0x0104_9694;
        const K0_G: u32 = 0x00fe_88e8;
        const WGT_SCALE_G: u32 = 0x0104_9698;
        const E18_SCALE_G: u32 = 0x00fe_8628;
        const KN_G: u32 = 0x00fe_8830;
        const WX_G: u32 = 0x00fe_8a24;
        const J_A4_G: u32 = 0x0104_969c;
        const J_A5_G: u32 = 0x0104_96a0;
        const J_A6_G: u32 = 0x0104_96a4;
        const J_A8_G: u32 = 0x0104_96a8;
        const K_A4_G: u32 = 0x0104_96ac;
        const K_A5_G: u32 = 0x0104_96b0;
        const K_A6_G: u32 = 0x0104_96b4;
        const K_A8_G: u32 = 0x016d_ceb4;
        const QK_G: u32 = 0x00fe_87e4;
        const ONE_BITS: u32 = 0x3f80_0000;
        const BONE_STRIDE: u32 = 64;
        const BONE_X: u32 = 0x30;
        const BONE_Y: u32 = 0x34;
        const BONE_Z: u32 = 0x38;
        const BONE_ARR: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::global::<u32>(va) as u32) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn wg32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wgf(va: u32, v: f32) {
            unsafe { wg32(va, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sqr(x: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(x)
        }
        #[inline(always)]
        fn fsqrt(x: f32) -> f32 {
            core::hint::black_box(x).sqrt()
        }
        /// One row of the bone array: the triple at row*64 + 0x30/0x34/0x38.
        #[inline(always)]
        unsafe fn bone_row(arr: u32, idx: u32) -> (f32, f32, f32) {
            unsafe {
                let base = arr.wrapping_add(idx.wrapping_mul(BONE_STRIDE));
                (rdf(base + BONE_X), rdf(base + BONE_Y), rdf(base + BONE_Z))
            }
        }

        // Entry: always callee 1, callee 2 unless the global byte is set.
        lf_checker_rt::callee_thiscall!(1, u32, this);
        if rd8(lf_checker_rt::global::<u8>(ENTRY_SEQ_G) as u32 + ENTRY_SEQ_BYTE) == 0 {
            lf_checker_rt::callee_thiscall!(2, u32, this);
        }
        // First virtual call (slot +0x24) through the object's own table.
        let vt = rd32(this + VTABLE_OFF);
        let v1: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt + 0x24) as usize) };
        if (v1(this) & 0xff) != 0 {
            // C region: virtual slot +8 on the object at +0x290, then one of
            // two early returns picked by the table entry's flag byte.
            let obj2 = rd32(this + OBJ2_PTR);
            let v3: extern "thiscall" fn(u32) -> u32 = unsafe {
                core::mem::transmute(rd32(rd32(obj2).wrapping_add(8)) as usize)
            };
            v3(obj2);
            let sx = rd16(this + TABLE_INDEX) as u16 as i16 as i32;
            let table = lf_checker_rt::global::<u32>(TABLE_G) as u32;
            let ent = rd32(table.wrapping_add((sx as u32).wrapping_mul(4)));
            if rd8(ent + FLAG_OFF) != 0 {
                let p = lf_checker_rt::callee_cdecl!(7, u32,);
                let w = rd32(p + 0xe98);
                return lf_checker_rt::callee_cdecl!(8, u32, w);
            }
            let v310 = rd32(this + CALL6_ARG2);
            let v294 = rd32(this + CALL6_ARG1);
            return lf_checker_rt::callee_cdecl!(6, u32, sx as u32, v294, v310);
        }
        // Second virtual call (slot +0x28); a zero answer returns it.
        let v2: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt + 0x28) as usize) };
        let r2 = v2(this);
        if (r2 & 0xff) == 0 {
            return r2;
        }
        // F region: table entry, matrix pointer, counter maze.
        let sx = rd16(this + TABLE_INDEX) as u16 as i16 as i32;
        let table = lf_checker_rt::global::<u32>(TABLE_G) as u32;
        let ebx = rd32(table.wrapping_add((sx as u32).wrapping_mul(4)));
        let sel = g32(SEL_G) as i32;
        let edx = if sel != -1 { sel } else { g32(EDX_A_G) as i32 };
        let e70ptr = rd32(rd32(this + STORE_CHAIN) + 4);
        let edi = rd32(this + MATRIX_PTR);
        let w2c = rd16(this + SCRIPT_WORD);
        let go_g = if edx > 0x14 {
            true
        } else {
            let mut eax = g32(EAX_G) as i32;
            let mut ecx = g32(ECX_G) as i32;
            if edx > 0x13 {
                if eax != -1 {
                    ecx = eax;
                }
                if ecx > ((w2c & 0x3f) as i32) {
                    true
                } else {
                    // edx == 0x14 here, so the check below falls to F3.
                    ecx = g32(ECX_G) as i32;
                    eax = g32(EAX_G) as i32;
                    let _ = (ecx, eax);
                    false
                }
            } else if edx < 6 {
                true
            } else if edx >= 7 {
                false
            } else {
                if eax != -1 {
                    ecx = eax;
                }
                ecx < ((w2c & 0x3f) as i32)
            }
        };
        let go_g = if go_g {
            true
        } else {
            // F3 float window: G unless glo <= x <= ghi... precisely, G
            // unless both comparisons fail, which is also where NaN lands.
            let x = mul((w2c as i32) as f32, gf(WIN_SCALE_G));
            gf(WIN_LO_G) > x || gf(WIN_HI_G) > x
        };
        if go_g && (g32(SETUP_FLAG_G) & 2) == 0 {
            lf_checker_rt::callee_thiscall!(
                9, u32, this, 0x32u32, 0x33u32, 1u32, 1u32, ONE_BITS, ebx, edi, 0u32
            );
            lf_checker_rt::callee_thiscall!(
                10, u32, this, 0x34u32, 0xffff_ffffu32, 0x35u32, 1u32, 0u32, 1u32,
                ONE_BITS, ebx, edi, 0u32
            );
            wrf(e70ptr + STORE_OFF, gf(STORE_VAL_G));
        }
        // H region: mode word picks K (not 1) or J.
        let bcc = rd32(ebx + PARAM_OFF);
        let idx_a = rd32(bcc + 0x24);
        if rd32(ebx + MODE_OFF) != 1 {
            // I guards: any negative index exits with the last loaded word.
            let i6c = rd32(bcc + 0x28);
            let i50 = rd32(bcc + 0x34);
            let i84 = rd32(bcc + 0x38);
            if (idx_a as i32) < 0 || (i6c as i32) < 0 || (i50 as i32) < 0 || (i84 as i32) < 0 {
                return i84;
            }
            // K region: four-bone average plus difference lengths.
            let arr1 = rd32(lf_checker_rt::callee_thiscall!(11, u32, this) + BONE_ARR);
            let (x1, y1, z1) = bone_row(arr1, idx_a);
            let arr2 = rd32(lf_checker_rt::callee_thiscall!(11, u32, this) + BONE_ARR);
            let (x2, y2, z2) = bone_row(arr2, i6c);
            let arr3 = rd32(lf_checker_rt::callee_thiscall!(11, u32, this) + BONE_ARR);
            let (x3, y3, z3) = bone_row(arr3, i50);
            let arr4 = rd32(lf_checker_rt::callee_thiscall!(11, u32, this) + BONE_ARR);
            let (x4, y4, z4) = bone_row(arr4, i84);
            let qk = gf(QK_G);
            let qx = mul(add(add(x3, add(x2, x1)), x4), qk);
            let qy = mul(add(add(y3, add(y2, y1)), y4), qk);
            let qz = mul(add(add(z3, add(z2, z1)), z4), qk);
            let zero = 0.0f32;
            let e0 = rdf(edi);
            let e4 = rdf(edi + 4);
            let e8 = rdf(edi + 8);
            let e10 = rdf(edi + 0x10);
            let e14 = rdf(edi + 0x14);
            let e18 = rdf(edi + 0x18);
            let e20 = rdf(edi + 0x20);
            let e24 = rdf(edi + 0x24);
            let e28 = rdf(edi + 0x28);
            let k0 = gf(K0_G);
            // B block: same blend shape as J.
            let b0 = sub(add(mul(e10, zero), mul(e0, zero)), mul(e20, k0));
            let b4 = sub(add(mul(e14, zero), mul(e4, zero)), mul(e24, k0));
            let b8 = sub(add(mul(e18, zero), mul(e8, zero)), mul(e28, k0));
            // C block: rows plus their zero multiples.
            let c0 = add(add(e10, mul(e0, zero)), mul(e20, zero));
            let c4 = add(add(e14, mul(e4, zero)), mul(e24, zero));
            let c8 = add(add(e18, mul(e8, zero)), mul(e28, zero));
            let mut blks = [0u32; 12];
            blks[0] = qx.to_bits();
            blks[1] = qy.to_bits();
            blks[2] = qz.to_bits();
            blks[3] = 0; // fill-defined tail
            blks[4] = b0.to_bits();
            blks[5] = b4.to_bits();
            blks[6] = b8.to_bits();
            blks[7] = 0; // fill-defined tail
            blks[8] = c0.to_bits();
            blks[9] = c4.to_bits();
            blks[10] = c8.to_bits();
            blks[11] = 0; // fill-defined tail
            let kn = gf(KN_G);
            // Lengths of difference triples; note each sum's lane order.
            let len_a = fsqrt(add(add(sqr(sub(y2, y4)), sqr(sub(x2, x4))), sqr(sub(z2, z4))));
            let len_b = fsqrt(add(add(sqr(sub(y1, y3)), sqr(sub(x1, x3))), sqr(sub(z1, z3))));
            let arg5 = mul(mul(add(len_a, len_b), kn), gf(K_A5_G));
            let arg5 = mul(arg5, kn);
            let len_c = fsqrt(add(add(sqr(sub(y3, y4)), sqr(sub(x3, x4))), sqr(sub(z3, z4))));
            let len_d = fsqrt(add(add(sqr(sub(y2, y1)), sqr(sub(x2, x1))), sqr(sub(z2, z1))));
            let arg4 = mul(mul(add(len_c, len_d), kn), gf(K_A4_G));
            let arg4 = mul(arg4, kn);
            let arg6 = mul(mul(mul(rdf(ebx + WEIGHT_OFF), gf(WX_G)), gf(K_A6_G)), kn);
            let bp = core::ptr::addr_of_mut!(blks) as u32;
            return lf_checker_rt::callee_cdecl!(
                13, u32, edi, bp, bp + 16, bp + 32, arg4.to_bits(), arg5.to_bits(),
                arg6.to_bits(), ONE_BITS, g32(K_A8_G)
            );
        }
        let idx_b = rd32(bcc + 0x34);
        if (idx_a as i32) < 0 {
            return idx_b;
        }
        if (idx_b as i32) < 0 {
            return idx_b;
        }
        // J region: two-bone blend.
        let arr1 = rd32(lf_checker_rt::callee_thiscall!(11, u32, this) + BONE_ARR);
        let (t1x, t1y, t1z) = bone_row(arr1, idx_a);
        let arr2 = rd32(lf_checker_rt::callee_thiscall!(11, u32, this) + BONE_ARR);
        let k1 = gf(K1_G);
        let k0 = gf(K0_G);
        let dk = sub(k0, k1);
        let (t2x, t2y, t2z) = bone_row(arr2, idx_b);
        let ix = add(mul(t1x, k1), mul(t2x, dk));
        let iy = add(mul(t1y, k1), mul(t2y, dk));
        let iz = add(mul(t1z, k1), mul(t2z, dk));
        let wgt = mul(rdf(ebx + WEIGHT_OFF), gf(WGT_SCALE_G));
        // Global accumulator: reload when its flag bit is set, else zero it
        // and set the bit.
        let flag = g32(ACC_FLAG_G);
        let (g0, g1, g2) = if flag & 1 != 0 {
            (gf(ACC0_G), gf(ACC1_G), gf(ACC2_G))
        } else {
            wg32(ACC_FLAG_G, flag | 1);
            wgf(ACC0_G, 0.0);
            wgf(ACC1_G, 0.0);
            wgf(ACC2_G, 0.0);
            (0.0, 0.0, 0.0)
        };
        let jx = add(ix, g0);
        let jy = add(iy, g1);
        let jz = add(iz, g2);
        let dx = sub(t1x, t2x);
        let dy = sub(t1y, t2y);
        let dz = sub(t1z, t2z);
        let zero = 0.0f32;
        let e0 = rdf(edi);
        let e4 = rdf(edi + 4);
        let e8 = rdf(edi + 8);
        let e10 = rdf(edi + 0x10);
        let e14 = rdf(edi + 0x14);
        let e18 = rdf(edi + 0x18);
        let e20 = rdf(edi + 0x20);
        let e24 = rdf(edi + 0x24);
        let e28 = rdf(edi + 0x28);
        let g8628 = gf(E18_SCALE_G);
        // B block: matrix rows blended against k0.
        let b0 = sub(add(mul(e10, zero), mul(e0, zero)), mul(e20, k0));
        let b4 = sub(add(mul(e14, zero), mul(e4, zero)), mul(e24, k0));
        let b8 = sub(add(mul(e18, g8628), mul(e8, zero)), mul(e28, k0));
        // A block: matrix rows plus their zero multiples.
        let a0 = add(add(mul(e0, zero), e10), mul(e20, zero));
        let a4 = add(add(mul(e4, zero), e14), mul(e24, zero));
        let a8 = add(add(mul(e8, zero), e18), mul(e28, zero));
        let mut blks = [0u32; 12];
        blks[0] = a0.to_bits();
        blks[1] = a4.to_bits();
        blks[2] = a8.to_bits();
        blks[3] = 0; // fill-defined tail (never-written frame slot)
        blks[4] = b0.to_bits();
        blks[5] = b4.to_bits();
        blks[6] = b8.to_bits();
        blks[7] = 0; // fill-defined tail
        blks[8] = jx.to_bits();
        blks[9] = jy.to_bits();
        blks[10] = jz.to_bits();
        blks[11] = 0; // fill-defined tail
        let kn = gf(KN_G);
        let arg4 = mul(mul(gf(J_A4_G), wgt), kn);
        let shade = add(fsqrt(add(add(sqr(dy), sqr(dx)), sqr(dz))), mul(wgt, gf(WX_G)));
        let arg5 = mul(mul(shade, gf(J_A5_G)), kn);
        let arg6 = mul(mul(gf(J_A6_G), wgt), kn);
        let arg8 = g32(J_A8_G);
        let bp = core::ptr::addr_of_mut!(blks) as u32;
        lf_checker_rt::callee_cdecl!(
            12, u32, edi, bp + 32, bp + 16, bp, arg4.to_bits(), arg5.to_bits(),
            arg6.to_bits(), ONE_BITS, arg8
        )
    }
});
