// original: 0x00ad4660 audio_stencil_blend_accumulate
// ---------------------------------------------------------------------------
// 0x00AD4660: interpolate a callee-filled stencil into accumulators.
// ---------------------------------------------------------------------------
// Two float arguments are scaled and rounded to grid indices through a
// magic-number rounding chain; depending on a null check and a float
// compare, one of four groups of three helper calls fills stencil slots,
// and the filled values are bilinearly blended into accumulator pointers
// (plus a normalized 3-vector store on the non-null path). Bitwise mask
// logic is transcribed exactly (never shortened to the idiom) so NaN and
// signed zeros propagate identically.
export!(cdecl, rw_00ad4660(fa: f32, fb: f32, acc: *mut u32, vec: *mut u32, acc2: *mut u32) -> u32 {
    unsafe {
        fn bits(x: f32) -> u32 {
            x.to_bits()
        }
        fn unbits(x: u32) -> f32 {
            f32::from_bits(x)
        }
        fn andp(a: f32, b: f32) -> f32 {
            unbits(bits(a) & bits(b))
        }
        fn xorp(a: f32, b: f32) -> f32 {
            unbits(bits(a) ^ bits(b))
        }
        fn orp(a: f32, b: f32) -> f32 {
            unbits(bits(a) | bits(b))
        }
        fn cmplt(a: f32, b: f32) -> f32 {
            unbits(if a < b { 0xFFFFFFFF } else { 0 })
        }
        // Imm6: not-less-or-equal: true iff a > b or either is NaN.
        // (Not imm5 not-less-than: they differ exactly on equality.)
        fn cmpnle(a: f32, b: f32) -> f32 {
            unbits(if a > b || a.is_nan() || b.is_nan() {
                0xFFFFFFFF
            } else {
                0
            })
        }
        fn cvt_trunc(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        let half = f32::from_bits(*global::<u32>(0xfe8830));
        let signm = f32::from_bits(*global::<u32>(0xfe8d1c));
        let magic = f32::from_bits(*global::<u32>(0xfe8cf8));
        let one = f32::from_bits(*global::<u32>(0xfe88e8));
        let two = f32::from_bits(*global::<u32>(0xfe8a24));

        // Callee out-slots, pre-zeroed exactly like the original's frame.
        let mut l0 = 0u32;
        let mut l1 = 0u32;
        let mut l4 = 0u32;
        let mut l5 = 0u32;
        let mut l6 = 0u32;
        let mut l7 = 0u32;

        let mut x7 = fa;
        x7 = x7 * half;
        let mut x5 = signm;
        let mut x4 = magic;
        let mut x2 = x5;
        x2 = andp(x2, x7);
        let mut x0 = x7;
        x0 = xorp(x0, x2);
        x0 = cmplt(x0, x4);
        let mut x6 = one;
        let mut x1 = x4;
        let mut x3 = x7;
        x1 = andp(x1, x0);
        x1 = orp(x1, x2);
        x3 = x3 + x1;
        x3 = x3 - x1;
        x0 = x3;
        x0 = x0 - x7;
        x0 = cmpnle(x0, x2);
        x2 = fb;
        x2 = x2 * half;
        x0 = andp(x0, x6);
        x3 = x3 - x0;
        x5 = andp(x5, x2);
        x0 = x2;
        x0 = xorp(x0, x5);
        x0 = cmplt(x0, x4);
        x1 = x2;
        x7 = x7 - x3;
        x3 = x3 * two;
        x4 = andp(x4, x0);
        x4 = orp(x4, x5);
        x1 = x1 + x4;
        let save_a = x7;
        let ebp0 = cvt_trunc(x3);
        x1 = x1 - x4;
        x0 = x1;
        x0 = x0 - x2;
        x0 = cmpnle(x0, x5);
        x0 = andp(x0, x6);
        x1 = x1 - x0;
        x2 = x2 - x1;
        x1 = x1 * two;
        let esi0 = cvt_trunc(x1);
        x0 = x2;
        let save_b = x2;
        let esi_saved = esi0;
        x0 = x0 + x7;

        if !vec.is_null() {
            let (mut ebp, mut esi, grp): (i32, i32, u32);
            if !(one > x0) {
                // Group 2 (comiss taken).
                esi = esi0.wrapping_add(2);
                let edi = ebp0.wrapping_add(2);
                ebp = ebp0;
                callee_cdecl!(4, u32, edi as u32, esi as u32, &mut l1 as *mut u32 as u32, &mut l0 as *mut u32 as u32);
                callee_cdecl!(5, u32, ebp as u32, esi as u32, &mut l4 as *mut u32 as u32, &mut l5 as *mut u32 as u32);
                callee_cdecl!(6, u32, edi as u32, esi_saved as u32, &mut l7 as *mut u32 as u32, &mut l6 as *mut u32 as u32);
                grp = 2;
            } else {
                // Group 1.
                ebp = ebp0;
                esi = esi0;
                callee_cdecl!(1, u32, ebp as u32, esi as u32, &mut l1 as *mut u32 as u32, &mut l0 as *mut u32 as u32);
                callee_cdecl!(2, u32, ebp.wrapping_add(2) as u32, esi as u32, &mut l4 as *mut u32 as u32, &mut l5 as *mut u32 as u32);
                callee_cdecl!(3, u32, ebp as u32, esi.wrapping_add(2) as u32, &mut l7 as *mut u32 as u32, &mut l6 as *mut u32 as u32);
                grp = 1;
            }
            let v_l1 = f32::from_bits(l1);
            let v_l4 = f32::from_bits(l4);
            let v_l7 = f32::from_bits(l7);
            let v_l5 = f32::from_bits(l5);
            if grp == 1 {
                let o0 = v_l1;
                let mut c3 = v_l4;
                let mut c4 = v_l7;
                let g6 = save_a;
                let g7 = save_b;
                c3 = c3 - o0;
                c4 = c4 - o0;
                let mut i1 = c3;
                i1 = i1 * g6;
                i1 = i1 + o0;
                let mut o = c4;
                o = o * g7;
                i1 = i1 + o;
                *acc = (f32::from_bits(*acc) + i1).to_bits();
                if !acc2.is_null() {
                    let q1 = v_l4;
                    let mut q2 = v_l1;
                    let mut q0 = v_l7;
                    q2 = q2 - q1;
                    q0 = q0 - q1;
                    q2 = q2 * g6;
                    q0 = q0 * g7;
                    q2 = q2 + q1;
                    q2 = q2 + q0;
                    *acc2 = q2.to_bits();
                }
                x5 = one;
            } else {
                let o0 = v_l1;
                let mut c3 = v_l4;
                let mut c4 = v_l7;
                x5 = one;
                let mut i1 = x5;
                i1 = i1 - save_a;
                x6 = x5;
                x6 = x6 - save_b;
                c3 = c3 - o0;
                c4 = c4 - o0;
                // The original spills (1-save_a) to the arg-1 slot here and
                // reloads it below; same value, no round trip needed.
                let w_a = i1;
                i1 = i1 * c3;
                i1 = i1 + o0;
                let mut o = x6;
                o = o * c4;
                i1 = i1 + o;
                *acc = (f32::from_bits(*acc) + i1).to_bits();
                if !acc2.is_null() {
                    let q1 = v_l4;
                    let mut q2 = v_l1;
                    let mut q0 = v_l7;
                    q2 = q2 - q1;
                    q0 = q0 - q1;
                    q2 = q2 * w_a;
                    q0 = q0 * x6;
                    q2 = q2 + q1;
                    q2 = q2 + q0;
                    *acc2 = q2.to_bits();
                }
            }
            // Common tail: scaled 3-vector store with normalize-or-zero.
            // The ucomiss+lahf+test+jp sequence jumps on even parity of
            // (ZF,PF), i.e. everywhere EXCEPT ordered-equal; and lahf
            // overwrites AH of the returned pointer. Both replicated exactly.
            x3 = v_l4 - v_l1;
            x4 = v_l7 - v_l1;
            x3 = x3 * half;
            x4 = x4 * half;
            *(vec.add(2) as *mut u32) = one.to_bits();
            *vec = x3.to_bits();
            let mut n1 = x4;
            n1 = n1 * x4;
            *(vec.add(1) as *mut u32) = x4.to_bits();
            let mut n0 = x3;
            n0 = n0 * x3;
            n1 = n1 + n0;
            n1 = n1 + x5;
            let tiny = f32::from_bits(*global::<u32>(0xfe8628));
            let (zf, pf, cf) = if n1.is_nan() || tiny.is_nan() {
                (true, true, true)
            } else if n1 == tiny {
                (true, false, false)
            } else if n1 < tiny {
                (false, false, true)
            } else {
                (false, false, false)
            };
            let ah: u32 = (if zf { 0x40 } else { 0 })
                | (if pf { 0x04 } else { 0 })
                | 0x02
                | (if cf { 0x01 } else { 0 });
            if ((ah & 0x44).count_ones() % 2) == 0 {
                let r0 = f32::sqrt(n1);
                n1 = x5;
                n1 = n1 / r0;
            } else {
                n1 = 0.0;
            }
            x3 = x3 * n1;
            x4 = x4 * n1;
            *vec = x3.to_bits();
            *(vec.add(1) as *mut u32) = x4.to_bits();
            *(vec.add(2) as *mut u32) = n1.to_bits();
            ((acc2 as u32) & 0xFFFF00FF) | (ah << 8)
        } else {
            let (mut ebp, mut esi, grp): (i32, i32, u32);
            if !(one > x0) {
                // Group 4.
                esi = esi0.wrapping_add(2);
                let edi = ebp0.wrapping_add(2);
                ebp = ebp0;
                callee_cdecl!(10, u32, edi as u32, esi as u32, &mut l1 as *mut u32 as u32, &mut l0 as *mut u32 as u32);
                callee_cdecl!(11, u32, ebp as u32, esi as u32, &mut l4 as *mut u32 as u32, &mut l5 as *mut u32 as u32);
                callee_cdecl!(12, u32, edi as u32, esi_saved as u32, &mut l7 as *mut u32 as u32, &mut l6 as *mut u32 as u32);
                grp = 4;
            } else {
                // Group 3.
                ebp = ebp0;
                esi = esi0;
                callee_cdecl!(7, u32, ebp as u32, esi as u32, &mut l1 as *mut u32 as u32, &mut l0 as *mut u32 as u32);
                callee_cdecl!(8, u32, ebp.wrapping_add(2) as u32, esi as u32, &mut l4 as *mut u32 as u32, &mut l5 as *mut u32 as u32);
                callee_cdecl!(9, u32, ebp as u32, esi.wrapping_add(2) as u32, &mut l7 as *mut u32 as u32, &mut l6 as *mut u32 as u32);
                grp = 3;
            }
            let v_l1 = f32::from_bits(l1);
            let v_l4 = f32::from_bits(l4);
            let v_l7 = f32::from_bits(l7);
            let mut j1;
            let mut j2;
            let mut j0;
            if grp == 3 {
                j1 = v_l1;
                j2 = v_l4;
                j0 = v_l7;
                j2 = j2 - j1;
                j0 = j0 - j1;
                j2 = j2 * save_a;
                j0 = j0 * save_b;
            } else {
                x5 = one;
                j1 = v_l1;
                j2 = v_l4;
                j0 = x5;
                j0 = j0 - save_a;
                x5 = x5 - save_b;
                j2 = j2 - j1;
                j2 = j2 * j0;
                j0 = v_l7;
                j0 = j0 - j1;
                j0 = j0 * x5;
            }
            j2 = j2 + j1;
            j2 = j2 + j0;
            *acc = (f32::from_bits(*acc) + j2).to_bits();
            acc as u32
        }
    }
});
