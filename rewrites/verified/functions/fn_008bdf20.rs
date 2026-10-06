// original: 0x008bdf20 ui_meter_update (proposed)

/// Update a UI meter from two polled integer sources, or enqueue a deferred
/// update when the worker context is present.
///
/// Entry probe: a flag helper decides whether this context is active (its
/// low byte; zero returns its full answer at once). Otherwise the thread
/// worker object is fetched through the TLS slot whose index the slot-index
/// global holds, and its context word at `+0x8CC` is tested. A zero context
/// word runs the immediate path below; a nonzero one allocates a 12-byte
/// request, stamps it with the request marker, mixes the sequence counter
/// into its second word (masked to 14 bits) and bumps the counter, stamps
/// the ready marker and the immediate-path entry address, hands it to the
/// queue helper, and returns that helper's answer (a failed allocation hands
/// a null pointer instead).
///
/// Immediate path: when the active word is exactly -1 (a signed equality
/// test) return the worker object at once. Otherwise two gate helpers
/// (each taking no arguments and answering in AL; no call site sets up ECX
/// for them) choose between
/// two pairs of source integers; the chosen pair is converted to floats and
/// passed with a shifted flag byte to the setup helper through two frame
/// slots. A float constant is then selected by two flag bytes (the first
/// constant survives only when the first byte differs from `0x6A` and the
/// third byte is zero), two table lookups fetch float pairs, and a draw helper
/// is invoked with twelve arguments (the two fetched floats, the constant,
/// four table addresses, two flag bytes, a state word and two fixed words)
/// before tail-jumping to the shared finish routine (no stack arguments).
///
/// Original: 0x008bdf20 (cdecl, no arguments). The immediate path is entered
/// by an internal jump and ends in a tail jump; the rewrite performs the
/// tail as a final call that never returns.
lf_checker_rt::export!(cdecl, rw_008bdf20() -> u32 {
    unsafe {
        const SLOT_INDEX: u32 = 0x17aba14;
        const CTX_WORD: u32 = 0x8cc;
        const SEQ_CTR: u32 = 0x10327a0;
        const REQ_MARK: u32 = 0xe7e048;
        const READY_MARK: u32 = 0xe7e080;
        const IMMED_ADDR: u32 = 0x8bdfa0;
        const ACTIVE_WORD: u32 = 0x1176760;
        const FLAG_BYTE: u32 = 0x1030b9c;
        const SRC_A0: u32 = 0x105c880;
        const SRC_A1: u32 = 0x105c87c;
        const SRC_B0: u32 = 0x105c884;
        const SRC_B1: u32 = 0x105c888;
        const CONST_A: u32 = 0xfe8874;
        const CONST_B: u32 = 0xfe88bc;
        const SEL0: u32 = 0x116c250;
        const SEL3: u32 = 0x116c253;
        const SEL_TAG: u8 = 0x6a;
        const FLAG_C0: u32 = 0x11766d0;
        const FLAG_C1: u32 = 0x11766d1;
        const STATE_W: u32 = 0x1176768;
        const TAB0: u32 = 0x1176770;
        const TAB1: u32 = 0x11767f0;
        const TAB2: u32 = 0x11764d0;
        const TAB3: u32 = 0x11765d0;
        const LOOKUP_TAG: u32 = 0x6a;

        let probe: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if (probe as u8) == 0 {
            return probe;
        }
        let slot: u32 = *lf_checker_rt::global::<u32>(SLOT_INDEX);
        let worker: u32 = lf_checker_rt::tls_slot(slot as usize);
        if *((worker + CTX_WORD) as *const u32) == 0 {
            // Immediate path.
            if *lf_checker_rt::global::<i32>(ACTIVE_WORD) == -1 {
                return worker;
            }
            let flag_shifted: u32 = (*lf_checker_rt::global::<u8>(FLAG_BYTE) as u32) << 24;
            let g1: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
            let mut src_a: u32 = *lf_checker_rt::global::<u32>(SRC_A0);
            if (g1 as u8) != 0 {
                src_a = *lf_checker_rt::global::<u32>(SRC_A1);
            }
            let g2: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
            let mut src_b: u32 = *lf_checker_rt::global::<u32>(SRC_B0);
            if (g2 as u8) != 0 {
                src_b = *lf_checker_rt::global::<u32>(SRC_B1);
            }
            let f_a: u32 = ((src_a as i32) as f32).to_bits();
            let f_b: u32 = ((src_b as i32) as f32).to_bits();
            let mut setup0: u32 = flag_shifted;
            let mut setup1: [u32; 4] = [0, f_a, f_b, 0];
            lf_checker_rt::callee_cdecl!(
                6,
                u32,
                setup1.as_mut_ptr() as u32,
                &mut setup0 as *mut u32 as u32
            );
            let mut pick: u32 = *lf_checker_rt::global::<u32>(CONST_A);
            // The first constant survives only when the first byte differs
            // from the tag and the third byte is zero; every other
            // combination overwrites it with the second constant.
            if !(*lf_checker_rt::global::<u8>(SEL0) != SEL_TAG
                && *lf_checker_rt::global::<u8>(SEL3) == 0)
            {
                pick = *lf_checker_rt::global::<u32>(CONST_B);
            }
            let got1: u32 =
                lf_checker_rt::callee_cdecl!(7, u32, setup1.as_mut_ptr() as u32, LOOKUP_TAG);
            let c0: u32 = *lf_checker_rt::global::<u8>(FLAG_C0) as u32;
            let c1: u32 = *lf_checker_rt::global::<u8>(FLAG_C1) as u32;
            let state: u32 = *lf_checker_rt::global::<u32>(STATE_W);
            let mut look2: [u32; 2] = [pick, 0];
            let got2: u32 =
                lf_checker_rt::callee_cdecl!(7, u32, look2.as_mut_ptr() as u32, LOOKUP_TAG);
            let float_b: u32 = *((got2 + 4) as *const u32);
            let float_a: u32 = *(got1 as *const u32);
            lf_checker_rt::callee_cdecl!(
                8,
                u32,
                float_a,
                float_b,
                pick,
                lf_checker_rt::relocated(TAB0),
                lf_checker_rt::relocated(TAB1),
                0u32,
                c0,
                state,
                c1,
                0xFFFFFFFFu32,
                lf_checker_rt::relocated(TAB2),
                lf_checker_rt::relocated(TAB3)
            );
            lf_checker_rt::callee_cdecl!(9, u32,)
        } else {
            // Deferred path: build and queue a request.
            let req: u32 = lf_checker_rt::callee_cdecl!(2, u32, 0x0cu32, 0u32);
            if req == 0 {
                let c: u32 = lf_checker_rt::callee_cdecl!(3, u32, 0u32);
                return c;
            }
            let w4: u32 = *((req + 4) as *const u32);
            *((req) as *mut u32) = lf_checker_rt::relocated(REQ_MARK);
            let seq: u32 = *lf_checker_rt::global::<u32>(SEQ_CTR);
            let masked: u32 = (w4 ^ seq) & 0x3fff;
            *((req + 4) as *mut u32) = w4 ^ masked;
            *lf_checker_rt::global::<u32>(SEQ_CTR) = seq.wrapping_add(1);
            *((req) as *mut u32) = lf_checker_rt::relocated(READY_MARK);
            *((req + 8) as *mut u32) = lf_checker_rt::relocated(IMMED_ADDR);
            let c: u32 = lf_checker_rt::callee_cdecl!(3, u32, req);
            c
        }
    }
});
