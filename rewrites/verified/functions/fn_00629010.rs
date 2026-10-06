// original: 0x00629010 tls_guarded_float_update (proposed)

/// Run one guarded float-block update for `this`, then tail-call its virtual
/// slot 4.
///
/// Takes two floats in the vector registers (XMM2, XMM1). The gate callee
/// decides (its low byte, plain `test`/`je` equality) whether the guarded
/// middle runs at all. The middle is reference-counted through TLS slot 0
/// `+0x0c`: the notify callee fires only when the count rises to exactly 1,
/// then nine float words (four constants, the two entry floats, and each
/// entry float plus the shared constant, in that operand order) plus a
/// pointer to a scratch word go to the block callee, and the count drops
/// again. When the count returns to 0 and the pending flag is set, the flush
/// callee fires and both pending words are cleared. The tail always runs: a
/// set flag byte at `this+0x10` with the emit flag set fires the emit callee
/// and clears one word; then virtual slot 4 of `this` is tail-called, the
/// flag byte is cleared, and the callee's answer is returned.
///
/// Original: 0x00629010 (thiscall, no stack arguments, vector-register float
/// arguments, returns the tail call's answer).
lf_checker_rt::export!(thiscall, rw_00629010(this: u32) -> u32 {
    unsafe {
        const GATE: u32 = 1;
        const NOTIFY: u32 = 2;
        const BLOCK: u32 = 3;
        const FLUSH: u32 = 4;
        const EMIT: u32 = 5;
        const TLS_COUNT: u32 = 0x0c;
        const FLAG_OFF: u32 = 0x10;
        const VT_SLOT4: u32 = 0x10;
        const ADD_CONST: u32 = 0x00fe88e8;
        const PENDING: u32 = 0x01b4f724;
        const PENDING2: u32 = 0x01b4f71c;
        const EMIT_FLAG: u32 = 0x017ed94b;
        const EMIT_WORD: u32 = 0x017f58e4;
        const TAIL_WORD: u32 = 0x017f58e0;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let fx2 = f32::from_bits(lf_checker_rt::xmm_word(2, 0));
        let fx1 = f32::from_bits(lf_checker_rt::xmm_word(1, 0));
        let gate: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if (gate & 0xff) != 0 {
            let tls0 = lf_checker_rt::tls_slot(0);
            let raised = rd32(tls0.wrapping_add(TLS_COUNT)).wrapping_add(1);
            ((tls0.wrapping_add(TLS_COUNT)) as *mut u32).write_unaligned(raised);
            if raised == 1 {
                lf_checker_rt::callee_cdecl!(NOTIFY, u32, 3, 6);
            }
            let c = f32::from_bits((lf_checker_rt::global::<u32>(ADD_CONST)).read());
            let f2 = add(fx2, c);
            let f3 = add(fx1, c);
            let mut scratch = 0xffffffffu32;
            lf_checker_rt::callee_cdecl!(
                BLOCK,
                u32,
                0xbf800000,
                0x3f800000,
                0x3f800000,
                0xbf800000,
                0x3dcccccd,
                fx1.to_bits(),
                fx2.to_bits(),
                f3.to_bits(),
                f2.to_bits(),
                &mut scratch as *mut u32 as u32
            );
            let lowered = rd32(tls0.wrapping_add(TLS_COUNT)).wrapping_sub(1);
            ((tls0.wrapping_add(TLS_COUNT)) as *mut u32).write_unaligned(lowered);
            if lowered == 0 {
                if (lf_checker_rt::global::<u32>(PENDING)).read() != 0 {
                    lf_checker_rt::callee_cdecl!(FLUSH, u32,);
                    (lf_checker_rt::global::<u32>(PENDING)).write(0);
                }
                (lf_checker_rt::global::<u32>(PENDING2)).write(0);
            }
        }
        if ((this.wrapping_add(FLAG_OFF)) as *const u8).read() != 0 {
            if (lf_checker_rt::global::<u8>(EMIT_FLAG)).read() != 0 {
                lf_checker_rt::callee_cdecl!(EMIT, u32,);
            }
            (lf_checker_rt::global::<u32>(EMIT_WORD)).write(0);
        }
        let vt = (this as *const u32).read_unaligned();
        let tail: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt.wrapping_add(VT_SLOT4)) as *const u32).read_unaligned()) as usize);
        (lf_checker_rt::global::<u32>(TAIL_WORD)).write(0);
        let ans = tail(this);
        ((this.wrapping_add(FLAG_OFF)) as *mut u8).write(0);
        ans
    }
});
