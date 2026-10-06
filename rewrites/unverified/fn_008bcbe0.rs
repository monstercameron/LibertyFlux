// original: 0x008bcbe0 ui_clip_update_large (proposed)

// STAGE 1: entry through the early-END path. The continue path past the
// level/flag gate (frame splat, table init, three loops, vtable calls) is
// Stage 2 and panics if reached; the Stage-1 contract steers every trial
// to END. The code below the gate is final; only the steering is staged.

/// Poll the display aspect, decide the clip level, and run or skip the
/// worker update (stage 1: early-END path only).
///
/// Asks the aspect helper (a thiscall taking 1, answering a float on the
/// x87 stack) for the aspect ratio, marks the pending byte, then derives a
/// level byte: unless the limit global is at or below 1175.0 the level is
/// the truncation of 255 minus (1175 minus the limit); at or below, the
/// level byte is zero. (Ordered `comiss` semantics throughout: `jbe` is
/// taken for NaN.) The thread worker object is fetched through the TLS slot
/// whose index the slot-index global holds: a zero context word calls the
/// brightness updater directly, a nonzero one allocates a 12-byte request,
/// stamps it, mixes the sequence counter into it and queues it with the
/// updater's address (a failed allocation queues null). A status helper
/// then decides a float block: the integer global converted to float,
/// scaled by the factor global, must exceed the first table lookup, and
/// the second lookup must exceed the blend in turn; both cannot hold at
/// once (strictly greater both ways), so the color helper and state-word
/// store after them are dead code, kept here for faithfulness. A tail
/// helper runs (its
/// answer is forced to 0xFF and discarded), an optional object helper runs
/// when its flag byte is set, and the function ends early when the level
/// byte is zero and the flag byte is 0xFF, returning the status helper's
/// answer. Any other combination continues past the gate (stage 2).
///
/// Original: 0x008bcbe0 (cdecl, no arguments; callee-saved esi/edi/ebx
/// preserved; 16-aligned frame with security cookie).
lf_checker_rt::export!(cdecl, rw_008bcbe0() -> u32 {
    unsafe {
        const STATE_OBJ: u32 = 0x118d7f0;
        const PENDING_BYTE: u32 = 0x1160b84;
        const LIMIT_WORD: u32 = 0x11609cc;
        const LIMIT_REF: u32 = 0xe7e0b0; // 1175.0
        const BASE_OFF: f32 = f32::from_bits(0x44b2c000); // 1430.0
        const SCALE: f32 = f32::from_bits(0x437f0000); // 255.0
        const SLOT_INDEX: u32 = 0x17aba14;
        const CTX_WORD: u32 = 0x8cc;
        const SEQ_CTR: u32 = 0x10327a0;
        const REQ_MARK: u32 = 0xe7e048;
        const READY_MARK: u32 = 0xe7e080;
        const UPDATER_ADDR: u32 = 0x8bd300;
        const SRC_INT: u32 = 0x18b7a8c;
        const SRC_SCALE: u32 = 0x17accf0;
        const STATE_WORD: u32 = 0x1030bc0;
        const OBJ_FLAG: u32 = 0x1161548;
        const OBJ_PTR: u32 = 0x1161548;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0f32 || x < -2147483648.0f32 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        *lf_checker_rt::global::<u8>(PENDING_BYTE) = 1;
        let _aspect: f32 =
            lf_checker_rt::callee_thiscall!(1, f32, lf_checker_rt::relocated(STATE_OBJ), 1u32);
        let g = f32::from_bits(*lf_checker_rt::global::<u32>(LIMIT_WORD));
        let cc = f32::from_bits(*lf_checker_rt::global::<u32>(LIMIT_REF));
        // Level byte starts at 0xFF (the flag store before the aspect call).
        let mut level: u8 = 0xff;
        if !(g > cc) {
            level = 0;
        } else if BASE_OFF > g {
            let t = sub(cc, g);
            let u = sub(SCALE, t);
            level = cvtt(u) as u8;
        }
        let slot: u32 = *lf_checker_rt::global::<u32>(SLOT_INDEX);
        let worker: u32 = lf_checker_rt::tls_slot(slot as usize);
        let has_ctx: bool = *((worker + CTX_WORD) as *const u32) != 0;
        if has_ctx {
            let req: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0x0cu32, 0u32);
            if req == 0 {
                lf_checker_rt::callee_cdecl!(3, u32, 0u32);
            } else {
                let w4: u32 = *((req + 4) as *const u32);
                *((req) as *mut u32) = lf_checker_rt::relocated(REQ_MARK);
                let seq: u32 = *lf_checker_rt::global::<u32>(SEQ_CTR);
                let masked: u32 = (w4 ^ seq) & 0x3fff;
                *((req + 4) as *mut u32) = w4 ^ masked;
                *lf_checker_rt::global::<u32>(SEQ_CTR) = seq.wrapping_add(1);
                *((req) as *mut u32) = lf_checker_rt::relocated(READY_MARK);
                *((req + 8) as *mut u32) = lf_checker_rt::relocated(UPDATER_ADDR);
                lf_checker_rt::callee_cdecl!(3, u32, req);
            }
        } else {
            lf_checker_rt::callee_cdecl!(4, u32,);
        }
        let status: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0u32);
        if (status as u8) != 0 {
            let src_i: u32 = *lf_checker_rt::global::<u32>(SRC_INT);
            let f_src: f32 = (src_i as i32) as f32;
            let scale: f32 = f32::from_bits(*lf_checker_rt::global::<u32>(SRC_SCALE));
            let blended: f32 = mul(f_src, scale);
            let mut look: [u32; 2] = [0, 0];
            let got1: u32 =
                lf_checker_rt::callee_cdecl!(6, u32, look.as_mut_ptr() as u32, 0u32);
            // Ordered comparisons; NaN exits via jbe. Note the first test
            // re-reads the blended float above, not the level byte.
            if blended > f32::from_bits(*(got1 as *const u32)) {
                let got_a: u32 =
                    lf_checker_rt::callee_cdecl!(6, u32, look.as_mut_ptr() as u32, 0x16u32);
                let fa: f32 = f32::from_bits(*(got_a as *const u32));
                if fa > blended {
                    let got_h: u32 =
                        lf_checker_rt::callee_cdecl!(7, u32, look.as_mut_ptr() as u32, 1u32);
                    *lf_checker_rt::global::<u32>(STATE_WORD) = *(got_h as *const u32);
                }
            }
        }
        lf_checker_rt::callee_cdecl!(8, u32,);
        // The original forces AL to 0xFF here, discarding the answer.
        let mut al: u8 = 0xff;
        if *lf_checker_rt::global::<u8>(OBJ_FLAG) != 0 {
            al = lf_checker_rt::callee_thiscall!(9, u32, lf_checker_rt::relocated(OBJ_PTR)) as u8;
        }
        if level != 0 || al != 0xff {
            unimplemented!("stage 2: continue path");
        }
        let r: u32 = lf_checker_rt::callee_cdecl!(5, u32, 0u32);
        lf_checker_rt::callee_cdecl!(10, u32,);
        r
    }
});
