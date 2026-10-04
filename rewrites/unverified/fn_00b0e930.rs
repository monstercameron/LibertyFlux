// original: 0x00B0E930 net_session_join_setup (proposed)

/// Resolve the join parameters for a network session object and submit them.
///
/// `arg` points to the session object: the chain at `+0x2C4`/`+0x25C` yields
/// an optional descriptor, `+0x6C` an optional aux object, `+0x219` a flag
/// byte. Returns -1 on every early-out and otherwise the handle produced by
/// the 13-argument build call (which may itself be -1).
///
/// Behaviour: when the descriptor chain resolves, look its `+0x18` key up;
/// unless the lookup's `+0xC` word is 1 the descriptor's `+0x60` word must be
/// non-zero to continue. A threshold call then picks how the count is formed:
/// below 0x3FFF (signed) it is the sign-extended word at `+0x86` of a second
/// lookup, otherwise twice that. The count starts at 1 when the chain is
/// empty. Next, when the aux object exists and its byte at `+0xE` is set, an
/// acquire call runs with (0, arg); otherwise a second acquire call runs with
/// (arg, flag == 0). A null answer, or a null `+0x25C` link in it, ends with
/// -1. When the mode global equals 2 and predicate A holds, the count becomes
/// 10 if the link's `+0x18` word is 0x1E. When the feature byte global is
/// clear, bit 0x200000 of the answer's `+0x24` word ends with -1, else a
/// finalize call runs and the result is -1 regardless. On the long path the
/// link's `+0x60` word must be non-zero or pass the lookup `+0xC == 1` check,
/// and its `+0x18` word must not be 0x2E; a register call runs, byte `+0x22A`
/// of the answer is set to 6, and a selector of 4 (predicate A true) or 5 is
/// formed. If predicate A then fails, control goes to the build path below.
/// Otherwise a probe call on the session must report false and predicate B
/// must report false, the first descriptor and the aux object must both be
/// present, and a 7-argument submit call runs with (arg, position, id word,
/// selector, count, 0, 0) where position is `+0x20` plus 0x30 when `+0x20`
/// is non-zero else the answer plus 0x10, and the id word is the
/// sign-extended word at `+0x2E`; the result is -1. The build path zeroes
/// three scratch words, runs the 13-argument build call with (slot, scratch
/// pointer, id word, two scratch words, 0, 0, 0, 0, 0, answer, 1, 1) where
/// slot mirrors the submit position, saves its answer, and returns it unless
/// predicate A holds and it is not -1, in which case a query call on the aux
/// object, a convert call on the saved answer (the original also pushes the
/// masked query word, but the callee pops only the top word and the caller
/// cleans the other, so it is observed nowhere), a post call on the notify
/// global with (convert answer - 0x578), and a flush call on the post result
/// run first; the saved answer is still returned.
///
/// The two scratch words and the scratch pointer passed to the build call
/// read stack the function never wrote; the contract defines that fill as
/// zero (see `stack_fill`), so the rewrite passes 0 and skips the pointer
/// comparison (contents snapped instead).
///
/// Original: 0x00B0E930 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00B0E930(arg: u32) -> u32 {
    unsafe {
        const SES_CHAIN: u32 = 0x2c4;
        const SES_AUX: u32 = 0x6c;
        const SES_FLAG: u32 = 0x219;
        const SES_ACQUIRE: u32 = 0x2b0;
        const LINK_INNER: u32 = 0x25c;
        const DESC_KEY: u32 = 0x18;
        const DESC_ALT: u32 = 0x60;
        const LOOKUP_TAG: u32 = 0xc;
        const LOOKUP_WORD: u32 = 0x86;
        const AUX_TAG: u32 = 0xe;
        const ANS_LINK: u32 = 0x25c;
        const ANS_FLAGS: u32 = 0x24;
        const ANS_MARK: u32 = 0x22a;
        const ANS_POS: u32 = 0x20;
        const ANS_IDW: u32 = 0x2e;
        const ANS_BASE: u32 = 0x10;
        const G_MODE: u32 = 0x011d_6fd4;
        const G_FEATURE: u32 = 0x0104_00b8;
        const G_NOTIFY: u32 = 0x018b_600c;
        const CALLEE_LOOKUP: u32 = 1;
        const CALLEE_THRESHOLD: u32 = 2;
        const CALLEE_ACQUIRE_A: u32 = 3;
        const CALLEE_ACQUIRE_B: u32 = 4;
        const CALLEE_PRED_A: u32 = 5;
        const CALLEE_FINALIZE: u32 = 6;
        const CALLEE_REGISTER: u32 = 7;
        const CALLEE_PROBE: u32 = 8;
        const CALLEE_PRED_B: u32 = 9;
        const CALLEE_SUBMIT: u32 = 10;
        const CALLEE_BUILD: u32 = 11;
        const CALLEE_QUERY: u32 = 12;
        const CALLEE_CONVERT: u32 = 13;
        const CALLEE_POST: u32 = 14;
        const CALLEE_FLUSH: u32 = 15;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16sx(a: u32) -> u32 {
            unsafe { (a as *const i16).read_unaligned() as i32 as u32 }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn pred_a() -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(CALLEE_PRED_A, u32,) }
        }

        let edi = arg;
        let chain = rd32(edi + SES_CHAIN);
        let first: u32 = if chain != 0 { rd32(chain + LINK_INNER) } else { 0 };
        let mut count: u32 = 1;
        if first != 0 {
            let p: u32 = lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, rd32(first + DESC_KEY));
            if rd32(p + LOOKUP_TAG) != 1 && rd32(first + DESC_ALT) == 0 {
                // skip the threshold block, count stays 1
            } else {
                let t: u32 = lf_checker_rt::callee_cdecl!(CALLEE_THRESHOLD, u32,);
                let p2: u32 =
                    lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, rd32(first + DESC_KEY));
                let w = rd16sx(p2 + LOOKUP_WORD);
                count = if (t as i32) >= 0x3fff { w.wrapping_add(w) } else { w };
            }
        }
        let aux = rd32(edi + SES_AUX);
        let answer: u32 = if aux != 0 && rd8(aux + AUX_TAG) != 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_ACQUIRE_A, u32, edi + SES_ACQUIRE, edi, 0)
        } else {
            let flag = if rd8(edi + SES_FLAG) == 0 { 1 } else { 0 };
            lf_checker_rt::callee_thiscall!(CALLEE_ACQUIRE_B, u32, edi + SES_ACQUIRE, edi, flag)
        };
        if answer == 0 {
            return 0xffff_ffff;
        }
        let link = rd32(answer + ANS_LINK);
        if link == 0 {
            return 0xffff_ffff;
        }
        if glob(G_MODE) == 2 && pred_a() & 0xff != 0 && rd32(link + DESC_KEY) == 0x1e {
            count = 10;
        }
        if (glob(G_FEATURE) >> 8) & 0xff == 0 {
            if rd32(answer + ANS_FLAGS) & 0x0020_0000 != 0 {
                return 0xffff_ffff;
            }
            lf_checker_rt::callee_thiscall!(CALLEE_FINALIZE, u32, answer);
            return 0xffff_ffff;
        }
        if rd32(link + DESC_ALT) == 0 {
            let p: u32 = lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, rd32(link + DESC_KEY));
            if rd32(p + LOOKUP_TAG) != 1 {
                return 0xffff_ffff;
            }
        }
        if rd32(link + DESC_KEY) == 0x2e {
            return 0xffff_ffff;
        }
        lf_checker_rt::callee_cdecl!(CALLEE_REGISTER, u32, answer);
        wr8(answer + ANS_MARK, 6);
        let selector: u32 = if pred_a() & 0xff != 0 { 4 } else { 5 };
        /// Shared tail: the 13-argument build call and the post chain.
        /// `scratch` stands in for the original's uninitialized scratch
        /// words: the contract's zero stack fill makes them read 0 on the
        /// original side, and the pointer comparison is skipped (contents
        /// snapped instead).
        unsafe fn build_tail(edi: u32, answer: u32) -> u32 {
            unsafe {
                let slot_base = rd32(answer + ANS_POS);
                let slot = if slot_base != 0 {
                    slot_base.wrapping_add(0x30)
                } else {
                    answer + ANS_BASE
                };
                let idw = rd16sx(answer + ANS_IDW);
                let mut scratch = [0u32; 2];
                let saved: u32 = lf_checker_rt::callee_cdecl!(
                    CALLEE_BUILD,
                    u32,
                    slot,
                    scratch.as_mut_ptr() as u32,
                    idw,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    answer,
                    1,
                    1
                );
                // The original pushes (masked, saved) for the convert call
                // but the callee pops only the top word; the masked word is
                // cleaned by the caller and observed nowhere, so only the
                // popped argument is modelled here.
                let a: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PRED_A, u32,);
                if a & 0xff == 0 || saved == 0xffff_ffff {
                    return saved;
                }
                let aux = rd32(edi + SES_AUX);
                let notify = glob(G_NOTIFY);
                let q: u32 = lf_checker_rt::callee_thiscall!(CALLEE_QUERY, u32, aux);
                let c: u32 = lf_checker_rt::callee_stdcall!(CALLEE_CONVERT, u32, saved);
                let _ = q & 0xffff;
                let post_arg = c.wrapping_sub(0x578);
                let pr: u32 = lf_checker_rt::callee_thiscall!(CALLEE_POST, u32, notify, post_arg);
                lf_checker_rt::callee_thiscall!(CALLEE_FLUSH, u32, pr);
                saved
            }
        }

        if pred_a() & 0xff == 0 {
            return build_tail(edi, answer);
        }
        let probe: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, edi);
        if probe & 0xff != 0 {
            return 0xffff_ffff;
        }
        let pb: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PRED_B, u32,);
        if pb & 0xff != 0 {
            return build_tail(edi, answer);
        }
        if first == 0 || aux == 0 {
            return 0xffff_ffff;
        }
        let base = rd32(answer + ANS_POS);
        let pos = if base != 0 { base.wrapping_add(0x30) } else { answer + ANS_BASE };
        let idw = rd16sx(answer + ANS_IDW);
        lf_checker_rt::callee_thiscall!(
            CALLEE_SUBMIT,
            u32,
            0x018e_cfb0,
            edi,
            pos,
            idw,
            selector,
            count,
            0,
            0
        );
        0xffff_ffff
    }
});
