// original: 0x00ae0ed0 input_state_create (proposed)

/// Allocate and construct the input-state object graph.
///
/// Takes one stack word (cdecl); only its low byte matters. It first
/// publishes three bit-copied float pairs and one integer from the
/// calibration globals, derives eleven working floats through exact
/// single-precision chains, and stages them with zeros and one constant
/// in a 72-byte frame. Seven allocations follow (the last only when the
/// flag byte is set), each storing null to its global when the allocator
/// answers null, otherwise running its constructor: the first block is
/// zero-filled, the second zero-filled plus linked, the rest take staged
/// frame pointers (and the flag or fixed data pointers) and their answers
/// land in their globals. One helper pair feeds a float to the fourth
/// constructor through the x87 register. Finally the first allocation is
/// linked to the other five answers (plus the conditional one when the
/// flag is set), which faults when the first allocation itself is null.
///
/// The staged frame is reproduced as a word array with the original's
/// exact offsets (every constructor argument points into it), and the
/// pushed data pointers are relocated image addresses, like the original's
/// relocated immediates. Nothing meaningful is returned.
///
/// Original: 0x00ae0ed0 (cdecl, one stack word, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ae0ed0(flag: u32) -> u32 {
    unsafe {
        const G_EFF0: u32 = 0x0118eff0;
        const G_EFF8: u32 = 0x0118eff8;
        const G_F000: u32 = 0x0118f000;
        const G_F008: u32 = 0x0118f008;
        const K_ADD1: u32 = 0x00fe86ec;
        const K_MUL_A: u32 = 0x00fe87c8;
        const K_SUB_A: u32 = 0x00fe870c;
        const K_ADD2: u32 = 0x00fe86d8;
        const K_MUL_B: u32 = 0x00fe881c;
        const K_MUL_C: u32 = 0x00fe880c;
        const K_SUB_B: u32 = 0x00fe8734;
        const K_SUB_C: u32 = 0x00fe8764;
        const G_B90: u32 = 0x01593b90;
        const G_B98: u32 = 0x01593b98;
        const G_BA0: u32 = 0x01593ba0;
        const G_BA8: u32 = 0x01593ba8;
        const G_F69: u32 = 0x0103f690;
        const G_B78: u32 = 0x01593b78;
        const G_B7C: u32 = 0x01593b7c;
        const G_B68: u32 = 0x01593b68;
        const G_B6C: u32 = 0x01593b6c;
        const G_B74: u32 = 0x01593b74;
        const G_B80: u32 = 0x01593b80;
        const G_B70: u32 = 0x01593b70;
        const P_A: u32 = 0x00ea72f0;
        const P_B: u32 = 0x00ea7300;
        const CALLEE_ALLOC: u32 = 1;
        const CALLEE_CTOR2: u32 = 2;
        const CALLEE_CTOR3: u32 = 3;
        const CALLEE_HELP1: u32 = 4;
        const CALLEE_HELP2: u32 = 5;
        const CALLEE_CTOR5: u32 = 6;
        const CALLEE_CTOR6: u32 = 7;
        const CALLEE_CTOR7: u32 = 8;
        const CALLEE_CTOR8: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        #[inline(always)]
        unsafe fn kf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }

        let bl = (flag & 0xff) as u8;
        // Calibration words; the pairs move as bits.
        let e0 = rd32(lf_checker_rt::relocated(G_EFF0));
        let a2b = rd32(lf_checker_rt::relocated(G_EFF0).wrapping_add(4));
        let a1b = rd32(lf_checker_rt::relocated(G_EFF8));
        let a3b = rd32(lf_checker_rt::relocated(G_EFF8).wrapping_add(4));
        let a0b = rd32(lf_checker_rt::relocated(G_F000));
        let f4b = rd32(lf_checker_rt::relocated(G_F000).wrapping_add(4));
        let i0 = rd32(lf_checker_rt::relocated(G_F008));
        let (a0, a1, a2, a3) = (
            f32::from_bits(a0b),
            f32::from_bits(a1b),
            f32::from_bits(a2b),
            f32::from_bits(a3b),
        );
        wr32(lf_checker_rt::relocated(G_B90), e0);
        wr32(lf_checker_rt::relocated(G_B90).wrapping_add(4), a2b);
        wr32(lf_checker_rt::relocated(G_B98), a1b);
        wr32(lf_checker_rt::relocated(G_B98).wrapping_add(4), a3b);
        wr32(lf_checker_rt::relocated(G_BA0), a0b);
        wr32(lf_checker_rt::relocated(G_BA0).wrapping_add(4), f4b);
        wr32(lf_checker_rt::relocated(G_BA8), i0);
        (lf_checker_rt::relocated(G_F69) as *mut u8).write(bl);
        // Working floats in the original's order.
        let k1 = kf(K_ADD1);
        let t_a1k1 = add(a1, k1);
        let t_a0k2 = mul(a0, kf(K_MUL_A));
        let t_a0a1 = add(a0, a1);
        let t_a3k3 = sub(a3, kf(K_SUB_A));
        let t_a2k1 = add(a2, k1);
        let t_x1 = add(t_a0k2, t_a1k1);
        let t_x0a = add(t_x1, kf(K_ADD2));
        let t_x1b = add(t_x1, kf(K_SUB_A));
        let t_a3k5 = mul(a3, kf(K_MUL_B));
        let t_a3k6 = mul(a3, kf(K_MUL_C));
        let t_x0b = add(t_a3k5, t_a2k1);
        let t_x0c = sub(t_a0a1, t_x1b);
        let t_x6b = add(t_a0a1, t_a0k2);
        let t_x0d = sub(t_x0c, kf(K_SUB_B));
        let t_x6c = sub(t_x6b, kf(K_SUB_C));
        let mut frame = [0u32; 18];
        frame[0x00] = a2b;
        frame[0x01] = t_x6c.to_bits();
        frame[0x02] = t_x0b.to_bits();
        frame[0x03] = t_x0d.to_bits();
        frame[0x04] = t_a2k1.to_bits();
        frame[0x05] = t_x0a.to_bits();
        frame[0x06] = t_a3k3.to_bits();
        frame[0x07] = t_a0k2.to_bits();
        frame[0x08] = t_a2k1.to_bits();
        frame[0x09] = t_a1k1.to_bits();
        frame[0x0a] = t_a3k6.to_bits();
        frame[0x0e] = a2b;
        frame[0x0f] = 0x3f59999a;
        frame[0x10] = t_a3k6.to_bits();
        let base = frame.as_mut_ptr() as u32;
        let at = |off: u32| base.wrapping_add(off);

        let b78g = lf_checker_rt::relocated(G_B78);
        let a1: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, 0x1cu32);
        if a1 == 0 {
            wr32(b78g, 0);
        } else {
            for i in 0..7u32 {
                wr32(a1.wrapping_add(i * 4), 0);
            }
            wr32(b78g, a1);
        }
        let a2o: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, 0x50u32);
        if a2o == 0 {
            wr32(lf_checker_rt::relocated(G_B7C), 0);
        } else {
            for i in 0..5u32 {
                wr32(a2o.wrapping_add(i * 4), 0);
            }
            wr32(a2o.wrapping_add(0x44), rd32(b78g));
            let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CTOR2, u32, a2o);
            wr32(lf_checker_rt::relocated(G_B7C), a2o);
        }
        let a3o: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, 0x2cu32);
        if a3o == 0 {
            wr32(lf_checker_rt::relocated(G_B68), 0);
        } else {
            let r: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_CTOR3, u32, a3o, at(0x10), at(0x08), rd32(b78g), flag
            );
            wr32(lf_checker_rt::relocated(G_B68), r);
        }
        let a4o: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, 0x110u32);
        if a4o == 0 {
            wr32(lf_checker_rt::relocated(G_B6C), 0);
        } else {
            let h1: u32 =
                lf_checker_rt::callee_stdcall!(CALLEE_HELP1, u32, at(0x20), at(0x18), rd32(b78g));
            let hf: f32 = lf_checker_rt::callee_stdcall!(CALLEE_HELP2, f32, h1);
            let r: u32 =
                lf_checker_rt::callee_thiscall!(CALLEE_CTOR5, u32, a4o, hf.to_bits());
            wr32(lf_checker_rt::relocated(G_B6C), r);
        }
        let a5o: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, 0x1e4u32);
        if a5o == 0 {
            wr32(lf_checker_rt::relocated(G_B74), 0);
        } else {
            let r: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_CTOR6, u32, a5o, at(0x00), at(0x28), rd32(b78g),
                lf_checker_rt::relocated(P_A)
            );
            wr32(lf_checker_rt::relocated(G_B74), r);
        }
        let a6o: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, 0x120u32);
        if a6o == 0 {
            wr32(lf_checker_rt::relocated(G_B80), 0);
        } else {
            let r: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_CTOR7, u32, a6o, at(0x38), at(0x30), rd32(b78g)
            );
            wr32(lf_checker_rt::relocated(G_B80), r);
        }
        if bl != 0 {
            let a7o: u32 = lf_checker_rt::callee_cdecl!(CALLEE_ALLOC, u32, 0xecu32);
            if a7o == 0 {
                wr32(lf_checker_rt::relocated(G_B70), 0);
            } else {
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_CTOR8, u32, a7o, at(0x00), at(0x40), rd32(b78g),
                    lf_checker_rt::relocated(P_B)
                );
                wr32(lf_checker_rt::relocated(G_B70), r);
            }
        }
        let link = rd32(b78g);
        wr32(link.wrapping_add(0x00), rd32(lf_checker_rt::relocated(G_B68)));
        wr32(link.wrapping_add(0x04), rd32(lf_checker_rt::relocated(G_B6C)));
        wr32(link.wrapping_add(0x0c), rd32(lf_checker_rt::relocated(G_B74)));
        wr32(link.wrapping_add(0x10), rd32(lf_checker_rt::relocated(G_B80)));
        wr32(link.wrapping_add(0x14), rd32(lf_checker_rt::relocated(G_B7C)));
        if bl != 0 {
            wr32(link.wrapping_add(0x08), rd32(lf_checker_rt::relocated(G_B70)));
        }
        0
    }
});
