// original: 0x00624140 net_handler_large (proposed)
// STATUS: retry kit - NOT RUN. Lane r-b311 designed the contract and this
// rewrite but ran out of time with two full verifications in flight.
// Next lane: add the include + mutant to lib.rs, build, probe 60, run 1200.

/// Dispatch a network object event to one of four handlers by object kind.
///
/// `this` is the session object, `arg0` is unused (accepted for the two-word
/// cleanup only), `arg1` is the event object. The object's slot-1 hook
/// answers a kind word that selects the handler. No return value.
///
/// Kind 0 (refresh): when the session state reads 2 or 3 and the two id
/// pairs match, fetch the object's fetch word through the fast hook; when it
/// is nonzero and equals the refresh gate, initialise a scratch record,
/// validate it, and submit it with the object words through the submit hook.
///
/// Kind 6 (session): after the same state gate and fetch, the session gate
/// word decides: on a match, initialise and validate a bigger record,
/// compare its state words against the session's, require the object's index
/// word to read -1, then run the two-phase commit (touch, refresh the id
/// pair, look up, construct); on a mismatch the predicate hook decides
/// between three sub-paths (resubmit like kind 0 against the refresh gate,
/// rebuild a small record against the retry gate and query it, or validate
/// the object and run the sibling drop-gamers routine from the batch).
///
/// Kinds 4 and 5 (timed): unless the time bound is clear, require the clock
/// hook's answer to fall within the bound of the session's stamp; then look
/// the object up, and either report it through the report hook (when both
/// predicates agree) or re-queue it.
///
/// Kind 2 (sweep): walk the session's node table (up to its count word);
/// for each present node run the touch hook, then the fast hook twice
/// around the sweep hook, skipping ahead on any zero answer.
///
/// Kind 1 and anything else ends the call. Original: 0x00624140 (thiscall,
/// two stack words).
lf_checker_rt::export!(thiscall, rw_00624140(this: u32, _arg0: u32, obj: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x50;
        const IDA0: u32 = 0xbf0;
        const IDA1: u32 = 0xbf4;
        const IDB0: u32 = 0xc30;
        const IDB1: u32 = 0xc34;
        const GST0: u32 = 0x540;
        const GST1: u32 = 0x544;
        const TSTAMP: u32 = 0x32f0;
        const NTAB: u32 = 0x2e24;
        const NCNT: u32 = 0x2ea4;
        const O_IDX: u32 = 0x20;
        const O_REC: u32 = 0x28;
        const G_REFRESH: u32 = 0x19f0a1c;
        const G_SESSION: u32 = 0x19f099c;
        const G_RETRY: u32 = 0x19f087c;
        const G_VALID: u32 = 0x19f097c;
        const G_BOUND: u32 = 0x18b74a4;
        const F_W10: u32 = 0x10;
        const F_BUF: u32 = 0x1a8;
        const F_G0: u32 = 0x98;
        const F_A0: u32 = 0xa0;
        const F_C0: u32 = 0x28;
        const F_P0: u32 = 0x60;
        const F_P1: u32 = 0x64;
        const F_S0: u32 = 0x18;
        const KIND_FETCH: u32 = 1;
        const FAST: u32 = 2;
        const INIT: u32 = 3;
        const VALIDATE: u32 = 4;
        const SUBMIT: u32 = 5;
        const INIT_BIG: u32 = 6;
        const VALID_BIG: u32 = 7;
        const PH_A: u32 = 8;
        const PH_B: u32 = 9;
        const TOUCH: u32 = 10;
        const PAIR: u32 = 11;
        const LOOKUP: u32 = 12;
        const CONSTRUCT: u32 = 13;
        const PRED: u32 = 14;
        const REBUILD: u32 = 15;
        const QUERY: u32 = 16;
        const INIT_V: u32 = 17;
        const VALID_O: u32 = 18;
        const SIBLING: u32 = 19;
        const CLOCK: u32 = 20;
        const FIND: u32 = 21;
        const PRED_S: u32 = 22;
        const REPORT: u32 = 23;
        const REQUEUE: u32 = 24;
        const SW_TOUCH: u32 = 25;
        const SW_FAST: u32 = 26;
        const SW_SWEEP: u32 = 27;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let mut fr = [0u8; 0x640];
        let fb = (&mut fr[0] as *mut u8) as u32;
        #[inline(always)]
        unsafe fn frd(fb: u32, off: u32) -> u32 {
            unsafe { ((fb.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn frw(fb: u32, off: u32, v: u32) {
            unsafe { ((fb.wrapping_add(off)) as *mut u32).write_unaligned(v) }
        }
        let g = |va: u32| rd32(lf_checker_rt::relocated(va));
        let state_ok = || {
            let st = rd32(this.wrapping_add(STATE));
            st >= 2 && st <= 3
        };

        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + 8) as usize);
        let kind = slot(obj);
        if kind == 0 {
            if !state_ok() {
                return 0;
            }
            if rd32(this.wrapping_add(IDA0)) != rd32(this.wrapping_add(IDB0)) {
                return 0;
            }
            if rd32(this.wrapping_add(IDA1)) != rd32(this.wrapping_add(IDB1)) {
                return 0;
            }
            let rec = rd32(obj.wrapping_add(O_REC));
            let r: u32 = lf_checker_rt::callee_fastcall!(
                FAST, u32, fb.wrapping_add(F_W10), rd32(rec.wrapping_add(0x40)),
                rd32(rec.wrapping_add(0x3c)));
            if r == 0 {
                return 0;
            }
            if g(G_REFRESH) != frd(fb, F_W10) {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(INIT, u32, fb.wrapping_add(F_BUF));
            let a: u32 = lf_checker_rt::callee_thiscall!(
                VALIDATE, u32, fb.wrapping_add(F_BUF), rd32(rec.wrapping_add(0x40)),
                rd32(rec.wrapping_add(0x3c)), fb.wrapping_add(F_BUF));
            if a & 0xff == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(
                SUBMIT, u32, this, fb.wrapping_add(F_BUF), rec.wrapping_add(0x2c),
                rd32(rec.wrapping_add(0x44)) & 0xffff);
            return 0;
        }
        if kind == 6 {
            if !state_ok() {
                return 0;
            }
            let rec = rd32(obj.wrapping_add(O_REC));
            let r: u32 = lf_checker_rt::callee_fastcall!(
                FAST, u32, fb.wrapping_add(F_W10), rd32(rec.wrapping_add(0x40)),
                rd32(rec.wrapping_add(0x3c)));
            if r == 0 {
                return 0;
            }
            let w10 = frd(fb, F_W10);
            if g(G_SESSION) == w10 {
                lf_checker_rt::callee_thiscall!(INIT_BIG, u32, fb.wrapping_add(F_G0));
                let a: u32 = lf_checker_rt::callee_thiscall!(
                    VALID_BIG, u32, fb.wrapping_add(F_G0),
                    rd32(rec.wrapping_add(0x40)), rd32(rec.wrapping_add(0x3c)), 0);
                if a & 0xff == 0 {
                    return 0;
                }
                if frd(fb, F_G0) != rd32(this.wrapping_add(GST0)) {
                    return 0;
                }
                if frd(fb, F_G0.wrapping_add(4)) != rd32(this.wrapping_add(GST1)) {
                    return 0;
                }
                if rd32(obj.wrapping_add(O_IDX)) != 0xffff_ffff {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(PH_A, u32, fb.wrapping_add(F_C0));
                lf_checker_rt::callee_thiscall!(PH_B, u32, fb.wrapping_add(F_C0));
                let t: u32 = lf_checker_rt::callee_thiscall!(
                    TOUCH, u32, this, fb.wrapping_add(F_A0));
                if t == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(PAIR, u32, fb.wrapping_add(F_C0), t);
                let h: u32 = lf_checker_rt::callee_thiscall!(
                    LOOKUP, u32, this, frd(fb, F_P0), frd(fb, F_P1));
                if h == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    CONSTRUCT, u32, this, h, fb.wrapping_add(F_G0));
                return 0;
            }
            let p: u32 = lf_checker_rt::callee_thiscall!(PRED, u32, this);
            if p & 0xff == 0 {
                if g(G_VALID) != w10 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(INIT_V, u32, fb.wrapping_add(F_G0));
                let a: u32 = lf_checker_rt::callee_thiscall!(
                    VALID_O, u32, this, rec, fb.wrapping_add(F_G0));
                if a & 0xff == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    SIBLING, u32, this, fb.wrapping_add(F_G0.wrapping_add(0x10)),
                    frd(fb, F_A0));
                return 0;
            }
            if g(G_REFRESH) == w10 {
                lf_checker_rt::callee_thiscall!(INIT, u32, fb.wrapping_add(F_BUF));
                let a: u32 = lf_checker_rt::callee_thiscall!(
                    VALIDATE, u32, fb.wrapping_add(F_BUF),
                    rd32(rec.wrapping_add(0x40)), rd32(rec.wrapping_add(0x3c)),
                    fb.wrapping_add(F_BUF));
                if a & 0xff == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    SUBMIT, u32, this, fb.wrapping_add(F_BUF), rec.wrapping_add(0x2c),
                    rd32(rec.wrapping_add(0x48)) & 0xffff);
                return 0;
            }
            if g(G_RETRY) != w10 {
                return 0;
            }
            frw(fb, F_S0, 0);
            frw(fb, F_S0.wrapping_add(4), 0);
            frw(fb, F_S0.wrapping_add(8), 0xffff_ffff);
            frw(fb, F_S0.wrapping_add(12), 0xffff_ffff);
            let a: u32 = lf_checker_rt::callee_thiscall!(
                REBUILD, u32, fb.wrapping_add(F_S0), rd32(rec.wrapping_add(0x40)),
                rd32(rec.wrapping_add(0x3c)), this);
            if a & 0xff == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(
                QUERY, u32, this, fb.wrapping_add(F_S0), rd32(obj.wrapping_add(O_IDX)));
            return 0;
        }
        if kind == 4 || kind == 5 {
            let bound = g(G_BOUND);
            if bound != 0 {
                let now: u32 = lf_checker_rt::callee_stdcall!(CLOCK, u32,);
                if now.wrapping_sub(rd32(this.wrapping_add(TSTAMP))) > bound {
                    return 0;
                }
            }
            let f: u32 = lf_checker_rt::callee_thiscall!(
                FIND, u32, this, rd32(obj.wrapping_add(O_IDX)));
            if f == 0 {
                return 0;
            }
            let s: u32 = lf_checker_rt::callee_thiscall!(PRED_S, u32, this);
            if s & 0xff == 0 {
                lf_checker_rt::callee_thiscall!(REQUEUE, u32, this, f);
                return 0;
            }
            let p: u32 = lf_checker_rt::callee_thiscall!(PRED, u32, this);
            if p & 0xff == 0 {
                lf_checker_rt::callee_thiscall!(REQUEUE, u32, this, f);
                return 0;
            }
            lf_checker_rt::callee_thiscall!(
                REPORT, u32, this, rd32(f.wrapping_add(0x40)),
                rd32(f.wrapping_add(0x44)), 0, 0);
            return 0;
        }
        if kind == 2 {
            let rec = rd32(obj.wrapping_add(O_REC));
            let n = rd32(this.wrapping_add(NCNT)) as i32;
            if n <= 0 {
                return 0;
            }
            // The node words sit inline at NTAB; the touch hook's argument
            // reuses the accumulator, which still holds the record except
            // after a full previous iteration (then it holds the fast
            // hook's answer, 0 in this model).
            let mut acc = rec;
            for i in 0..n as usize {
                let node = rd32(this.wrapping_add(NTAB).wrapping_add((i * 4) as u32));
                if node == 0 {
                    continue;
                }
                let t: u32 = lf_checker_rt::callee_thiscall!(
                    SW_TOUCH, u32, node.wrapping_add(0x48),
                    acc.wrapping_add(0x2c));
                acc = rec;
                if t & 0xff == 0 {
                    continue;
                }
                lf_checker_rt::callee_thiscall!(
                    SW_FAST, u32, 0, rd32(rec.wrapping_add(0x3c)));
                let s: u32 = lf_checker_rt::callee_thiscall!(
                    SW_SWEEP, u32, node.wrapping_add(0x48));
                if s & 0xff == 0 {
                    continue;
                }
                lf_checker_rt::callee_thiscall!(
                    SW_FAST, u32, node.wrapping_add(0x58), rd32(rec.wrapping_add(0x3c)));
                acc = 0;
            }
            return 0;
        }
        0
    }
});
