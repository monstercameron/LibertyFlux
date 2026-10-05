// original: 0x00dc2f10 ped_task_list_refine (proposed)

/// Refine a ped task list in place: copy its records to scratch buffers,
/// repeatedly pick neighbouring records whose separation passes two
/// threshold tests, validate each candidate pair through a query/lookup/
/// solve chain, and write the surviving records back.
///
/// `arg` points to the task-list object: a flag word at `+0x14` (bit 9 is
/// forwarded to the verify callee), the record count at `+0x1c`, 16-byte
/// records at `+0x90`, per-record indices at `+0x1a0` and a parallel array
/// at `+0x1e4` that is cleared on write-back. `this` is only forwarded to
/// the prepare/test/verify callees and is never dereferenced here.
///
/// Behaviour: lists with fewer than 3 records are returned unchanged. A
/// one-time initialisation converts a file double constant to the float at
/// `G14` unless the flag at `FLAG` is already set. Two scratch buffers are
/// allocated (512 and 128 bytes); the main loop scans neighbouring records
/// (up to 15 pairs), keeps pairs whose difference norms reach `THRESH`
/// (0.1225) and whose scaled dot product is below `G14`, derives ten blend
/// coefficients from the two norms, and runs five validation passes per
/// pair. A passing pair overwrites the record, inserts a zero index and
/// appends a derived record through the append callee. At most 16 records
/// are copied back, the count is updated, both buffers are freed and the
/// second free's answer is returned. The cookie check runs on every path
/// including the early return.
///
/// Calling convention: thiscall, one stack word. The float operation order
/// is the original's. Four frame words passed to the solve callee are read
/// before ever being written; the contract fills uninitialised stack with
/// zero, so they read as zero here.
lf_checker_rt::export!(thiscall, rw_00dc2f10(this: u32, arg: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x017A_6B18;
        const G14: u32 = 0x017A_6B14;
        const INITDBL: u32 = 0x00EA_E7E0;
        const THRESH: u32 = 0x00EF_44BC;
        const K15: u32 = 0x00FE_87B4;
        const K25: u32 = 0x00FE_87E4;
        const K50: u32 = 0x00FE_8830;
        const K75: u32 = 0x00FE_888C;
        const K85: u32 = 0x00FE_88B0;
        const ONE: u32 = 0x00FE_88E8;
        const OFF_FLAGS: u32 = 0x14;
        const OFF_COUNT: u32 = 0x1c;
        const OFF_REC: u32 = 0x90;
        const OFF_IDX: u32 = 0x1a0;
        const OFF_AUX: u32 = 0x1e4;
        const REC_STRIDE: u32 = 16;
        const QRY_FAIL: u32 = 0xfff;
        const SOLV_FAIL: u32 = 0xffff;
        const FIVE: f32 = 5.0;

        const C_INIT: u32 = 1;
        const C_MALLOC_A: u32 = 2;
        const C_MALLOC_B: u32 = 3;
        const C_SQRT: u32 = 4;
        const C_QRY_A: u32 = 5;
        const C_LOOK_A: u32 = 6;
        const C_SOLV_A: u32 = 7;
        const C_QRY_B: u32 = 8;
        const C_LOOK_B: u32 = 9;
        const C_SOLV_B: u32 = 10;
        const C_PREP: u32 = 11;
        const C_TEST: u32 = 12;
        const C_VERIFY: u32 = 13;
        const C_APPEND: u32 = 14;
        const C_FREE: u32 = 15;
        const C_COOKIE: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn gf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(a))) }
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

        let count0 = rd32(arg.wrapping_add(OFF_COUNT));
        if count0 < 3 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
            return arg;
        }

        let flag = rd32(lf_checker_rt::relocated(FLAG));
        if flag & 1 == 0 {
            let d = (lf_checker_rt::relocated(INITDBL) as *const f64).read_unaligned();
            lf_checker_rt::callee_thiscall!(C_INIT, u32, this);
            wr32(lf_checker_rt::relocated(G14), (d as f32).to_bits());
            wr32(lf_checker_rt::relocated(FLAG), flag | 1);
        }

        let buf_a = lf_checker_rt::callee_cdecl!(C_MALLOC_A, u32, 0x200u32);
        let buf_b = lf_checker_rt::callee_cdecl!(C_MALLOC_B, u32, 0x80u32);

        // Fill: copy records and indices into the scratch buffers.
        let mut slot50: u32 = 0x20_0000;
        let mut i3c: u32 = 0;
        let mut slot40: u32 = 0;
        if count0 != 0 {
            let mut src_idx = arg.wrapping_add(OFF_IDX);
            let mut rec_ptr = arg.wrapping_add(OFF_REC).wrapping_add(8);
            let mut n: u32 = 0;
            while n != count0 {
                let ax = i3c as u16;
                let x0 = rdf(rec_ptr.wrapping_sub(4));
                let x1 = rdf(rec_ptr);
                let cx = ax as u32;
                let ax1 = ax.wrapping_add(1);
                slot50 = (slot50 & 0xffff_0000) | ax1 as u32;
                let dst = buf_a.wrapping_add(cx << 4);
                rec_ptr = rec_ptr.wrapping_add(REC_STRIDE);
                i3c = (i3c & 0xffff_0000) | ax1 as u32;
                wr32(dst, rd32(rec_ptr.wrapping_sub(0x18)));
                wrf(dst.wrapping_add(4), x0);
                wrf(dst.wrapping_add(8), x1);
                wr32(dst.wrapping_add(0xc), rd32(rec_ptr.wrapping_sub(0xc)));
                let j = slot40;
                let cx2 = (j as u16) as u32;
                slot40 = (j & 0xffff_0000) | (j as u16).wrapping_add(1) as u32;
                let v = rd32(src_idx);
                src_idx = src_idx.wrapping_add(4);
                wr32(buf_b.wrapping_add(cx2.wrapping_mul(4)), v);
                n = n.wrapping_add(1);
            }
        }

        let thresh = gf(THRESH);
        let k15 = gf(K15);
        let k25 = gf(K25);
        let k50 = gf(K50);
        let k75 = gf(K75);
        let k85 = gf(K85);
        let one = gf(ONE);

        // Main loop over neighbouring record pairs.
        let mut n_a = i3c;
        if ((n_a as u16) as u32).wrapping_sub(1) as i32 <= 1 {
            // Fewer than 3 records buffered: straight to write-back.
        } else {
            let mut slot54: u32 = 1;
            let mut edxv: u32 = 2;
            let mut slot34 = buf_b.wrapping_add(8);
            let mut ediv: u32 = 0x10;
            loop {
                if (ediv as i32) >= 0x100 {
                    break;
                }
                if rd16(buf_b.wrapping_add(slot54.wrapping_mul(4))) != 0 {
                    slot34 = slot34.wrapping_add(4);
                } else {
                    let d0 = sub(rdf(buf_a.wrapping_add(ediv)), rdf(buf_a.wrapping_add(ediv).wrapping_sub(0x10)));
                    let e0 = sub(rdf(buf_a.wrapping_add(ediv).wrapping_add(0x14)), rdf(buf_a.wrapping_add(ediv).wrapping_add(4)));
                    let d1 = sub(rdf(buf_a.wrapping_add(ediv).wrapping_add(4)), rdf(buf_a.wrapping_add(ediv).wrapping_sub(0xc)));
                    let d2 = sub(rdf(buf_a.wrapping_add(ediv).wrapping_add(8)), rdf(buf_a.wrapping_add(ediv).wrapping_sub(8)));
                    let e1 = sub(rdf(buf_a.wrapping_add(ediv).wrapping_add(0x10)), rdf(buf_a.wrapping_add(ediv)));
                    let e2 = sub(rdf(buf_a.wrapping_add(ediv).wrapping_add(0x18)), rdf(buf_a.wrapping_add(ediv).wrapping_add(8)));
                    let n2 = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
                    let n1 = add(add(mul(e0, e0), mul(e1, e1)), mul(e2, e2));
                    if thresh > n2 || thresh > n1 {
                        slot34 = slot34.wrapping_add(4);
                    } else {
                        let s1: f32 = lf_checker_rt::callee_cdecl!(C_SQRT, f32, n2.to_bits());
                        let d0s = mul(d0, s1);
                        let d1s = mul(d1, s1);
                        let d2s = mul(d2, s1);
                        let s2: f32 = lf_checker_rt::callee_cdecl!(C_SQRT, f32, n1.to_bits());
                        let e0s = mul(e0, s2);
                        let e1s = mul(e1, s2);
                        let e2s = mul(e2, s2);
                        let dot = add(add(mul(e0s, d1s), mul(e1s, d0s)), mul(e2s, d2s));
                        let g14 = gf(G14);
                        if g14 > dot {
                            let n2s = core::hint::black_box(n2).sqrt();
                            let n1s = core::hint::black_box(n1).sqrt();
                            let mut c = [0.0f32; 10];
                            c[0] = mul(n2s, k50);
                            c[1] = mul(n2s, k75);
                            c[2] = mul(n2s, k85);
                            let t = sub(n2s, one);
                            c[3] = if t > one { t } else { one };
                            let t = sub(n2s, k50);
                            c[4] = if t > one { t } else { one };
                            c[5] = mul(n1s, k50);
                            c[6] = mul(n1s, k25);
                            c[7] = mul(n1s, k15);
                            c[8] = if n1s > one { one } else { n1s };
                            c[9] = if n1s > k50 { k50 } else { n1s };
                            // Five validation passes over the coefficient pairs.
                            let mut flag_b: u8 = 0;
                            let mut cl: u8 = 0;
                            let mut succeeded = false;
                            let mut sub_i: u32 = 0;
                            // Saved across the chain for the success block.
                            let mut sw = [0.0f32; 4];
                            let mut sv = [0.0f32; 4];
                            loop {
                                let ca = c[(sub_i / 4) as usize];
                                let cb = c[(5 + sub_i / 4) as usize];
                                let w0 = add(mul(d0s, ca), rdf(buf_a.wrapping_add(ediv).wrapping_sub(0x10)));
                                let w1 = add(mul(d1s, ca), rdf(buf_a.wrapping_add(ediv).wrapping_sub(0xc)));
                                let w2 = add(mul(d2s, ca), rdf(buf_a.wrapping_add(ediv).wrapping_sub(8)));
                                let w3 = 0.0f32;
                                let v0 = add(mul(e1s, cb), rdf(buf_a.wrapping_add(ediv)));
                                let v1 = add(mul(e0s, cb), rdf(buf_a.wrapping_add(ediv).wrapping_add(4)));
                                let v2 = add(mul(e2s, cb), rdf(buf_a.wrapping_add(ediv).wrapping_add(8)));
                                let v3 = 0.0f32;
                                sw = [w0, w1, w2, w3];
                                sv = [v0, v1, v2, v3];
                                let r1: u32 = lf_checker_rt::callee_cdecl!(C_QRY_A, u32, sw.as_ptr() as u32);
                                if r1 == QRY_FAIL {
                                    cl = flag_b;
                                } else {
                                    let o1: u32 = lf_checker_rt::callee_cdecl!(C_LOOK_A, u32, r1);
                                    if o1 == 0 {
                                        cl = flag_b;
                                    } else {
                                        let w2c = add(w2, one);
                                        let d2a = [w0, w1, w2c];
                                        let z = [0u32; 4];
                                        let q1: u32 = lf_checker_rt::callee_thiscall!(C_SOLV_A, u32, o1, d2a.as_ptr() as u32, z.as_ptr() as u32, FIVE.to_bits());
                                        if q1 == SOLV_FAIL {
                                            cl = flag_b;
                                        } else {
                                            let p1 = rd32(o1.wrapping_add(0x6c)).wrapping_add(q1.wrapping_add(q1 << 2) << 3);
                                            let r2: u32 = lf_checker_rt::callee_cdecl!(C_QRY_B, u32, sv.as_ptr() as u32);
                                            if r2 == QRY_FAIL {
                                                cl = flag_b;
                                            } else {
                                                let o2: u32 = lf_checker_rt::callee_cdecl!(C_LOOK_B, u32, r2);
                                                if o2 == 0 {
                                                    cl = flag_b;
                                                } else {
                                                    let v2c = add(v2, one);
                                                    let d2b = [v0, v1, v2c];
                                                    let z = [0u32; 4];
                                                    let q2: u32 = lf_checker_rt::callee_thiscall!(C_SOLV_B, u32, o2, d2b.as_ptr() as u32, z.as_ptr() as u32, FIVE.to_bits());
                                                    if q2 == SOLV_FAIL {
                                                        cl = flag_b;
                                                    } else {
                                                        let p2 = rd32(o2.wrapping_add(0x6c)).wrapping_add(q2.wrapping_add(q2 << 2) << 3);
                                                        lf_checker_rt::callee_thiscall!(C_PREP, u32, this);
                                                        let t: u32 = lf_checker_rt::callee_thiscall!(C_TEST, u32, this, sw.as_ptr() as u32, sv.as_ptr() as u32, p2, p1, 0u32);
                                                        flag_b = t as u8;
                                                        if flag_b == 0 {
                                                            cl = 0;
                                                        } else {
                                                            let bit = (rd32(arg.wrapping_add(OFF_FLAGS)) >> 9) & 1;
                                                            let v: u32 = lf_checker_rt::callee_thiscall!(C_VERIFY, u32, this, sw.as_ptr() as u32, sv.as_ptr() as u32, bit);
                                                            flag_b = v as u8;
                                                            if flag_b == 0 {
                                                                cl = 0;
                                                            } else {
                                                                succeeded = true;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if succeeded {
                                    break;
                                }
                                sub_i = sub_i.wrapping_add(4);
                                if sub_i >= 0x14 {
                                    break;
                                }
                            }
                            if succeeded || cl != 0 {
                                wrf(buf_a.wrapping_add(ediv), sw[0]);
                                wrf(buf_a.wrapping_add(ediv).wrapping_add(4), sw[1]);
                                wrf(buf_a.wrapping_add(ediv).wrapping_add(8), sw[2]);
                                wr32(buf_a.wrapping_add(ediv).wrapping_add(0xc), sw[3].to_bits());
                                wr16(buf_b.wrapping_add(slot54.wrapping_mul(4)), 0);
                                wr8(buf_b.wrapping_add(slot54.wrapping_mul(4)).wrapping_add(2), 0);
                                let mut bufa_slot = buf_a;
                                let ap: u32 = lf_checker_rt::callee_thiscall!(C_APPEND, u32, core::ptr::addr_of_mut!(bufa_slot) as u32, edxv);
                                wrf(ap, sv[0]);
                                wrf(ap.wrapping_add(4), sv[1]);
                                wrf(ap.wrapping_add(8), sv[2]);
                                wrf(ap.wrapping_add(0xc), sv[3]);
                                let mut c2 = (slot40 as u16) as u32;
                                while (c2 as i32) > (edxv as i32) {
                                    let mv = rd32(buf_b.wrapping_add(c2.wrapping_mul(4)).wrapping_sub(4));
                                    wr32(buf_b.wrapping_add(c2.wrapping_mul(4)), mv);
                                    c2 = c2.wrapping_sub(1);
                                }
                                slot40 = (slot40 & 0xffff_0000) | (slot40 as u16).wrapping_add(1) as u32;
                                wr16(slot34, 0);
                                wr8(slot34.wrapping_add(2), 0);
                                n_a = (slot50 as u16) as u32;
                            }
                            slot34 = slot34.wrapping_add(4);
                        } else {
                            slot34 = slot34.wrapping_add(4);
                        }
                    }
                }
                slot54 = slot54.wrapping_add(1);
                edxv = edxv.wrapping_add(1);
                ediv = ediv.wrapping_add(0x10);
                let bound = ((n_a as u16) as u32).wrapping_sub(1);
                if !((slot54 as i32) < (bound as i32)) {
                    break;
                }
            }
        }

        // Write-back: copy at most 16 records, indices and cleared aux words.
        let s24 = (slot50 >> 16) as u16 as u32;
        let nlow = (n_a as u16) as u32;
        let m = if (nlow as i32) < 16 { nlow } else { 16 };
        if m != 0 {
            let mut i: u32 = 0;
            while i < m {
                wr32(arg.wrapping_add(OFF_REC).wrapping_add(i.wrapping_mul(16)), rd32(buf_a.wrapping_add(i.wrapping_mul(16))));
                wrf(arg.wrapping_add(OFF_REC).wrapping_add(i.wrapping_mul(16)).wrapping_add(4), rdf(buf_a.wrapping_add(i.wrapping_mul(16)).wrapping_add(4)));
                wrf(arg.wrapping_add(OFF_REC).wrapping_add(i.wrapping_mul(16)).wrapping_add(8), rdf(buf_a.wrapping_add(i.wrapping_mul(16)).wrapping_add(8)));
                wr32(arg.wrapping_add(OFF_REC).wrapping_add(i.wrapping_mul(16)).wrapping_add(0xc), rd32(buf_a.wrapping_add(i.wrapping_mul(16)).wrapping_add(0xc)));
                wr32(arg.wrapping_add(OFF_IDX).wrapping_add(i.wrapping_mul(4)), rd32(buf_b.wrapping_add(i.wrapping_mul(4))));
                wr32(arg.wrapping_add(OFF_AUX).wrapping_add(i.wrapping_mul(4)), 0);
                i = i.wrapping_add(1);
            }
        }
        wr32(arg.wrapping_add(OFF_COUNT), m);
        let mut fr: u32 = lf_checker_rt::callee_cdecl!(C_FREE, u32, buf_b);
        if (s24 as u16) != 0 {
            fr = lf_checker_rt::callee_cdecl!(C_FREE, u32, buf_a);
        }
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        fr
    }
});
