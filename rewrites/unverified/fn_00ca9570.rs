// original: 0x00ca9570 CEventHandler::vf52
/// Answer a weapon-threat event: identify the subject through the event's
/// virtual slot, then either build a disarm task chain or route an
/// observer object through an alternate task, depending on the event kind.
///
/// `handler` points to the event handler (`+0x04` holds its ped, `+0x08`
/// and `+0x0c` receive tasks on different paths). `event` is the event
/// record (`+0x10` is the kind: 0xc8 stores zero and returns the subject,
/// 0x137 takes the observer path with the second stack argument, 0x76c
/// takes the main path, anything else returns the subject with nothing
/// stored). The third stack argument is not read.
///
/// The main path requires a subject mode mask of 0xc0, then optionally
/// runs a weapon check (a lookup, a positive power float, a weapon-info
/// mode pick) before allocating a pool slot, threading it through a
/// follow-up call, setting a flag bit on the result, and, when the
/// handler's second slot is clear and a proximity check passes, building
/// the final task from a two-call sequence whose first call leaves stack
/// words the second consumes. Two paths dereference null (a missing
/// power block, a null pool slot): both sides fault identically there.
/// The observer path validates the object (kind 0x19f, non-negative tag)
/// and either finishes directly or falls into the alternate task chain
/// (a predicate, an allocation, a two-call build).
///
/// Original: 0x00ca9570 (thiscall, three stack words; the third is not read).
lf_checker_rt::export!(thiscall, rw_00ca9570(handler: u32, event: u32, observer: u32, _a3: u32) -> u32 {
    unsafe {
        const HANDLER_PED: u32 = 0x04;
        const HANDLER_ALT: u32 = 0x08;
        const HANDLER_TASK: u32 = 0x0c;
        const EVT_KIND: u32 = 0x10;
        const EVT_WEAPON: u32 = 0x48;
        const KIND_STORE0: u32 = 0xc8;
        const KIND_OBSERVER: u32 = 0x137;
        const KIND_MAIN: u32 = 0x76c;
        const EVT_SUBJECT: u32 = 0x34;
        const SUBJ_POS: u32 = 0x20;
        const POS_ARG: u32 = 0x30;
        const SUBJ_MODE: u32 = 0x28;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_WANT: u32 = 0xc0;
        const SUBJ_FLAG: u32 = 0x219;
        const SUBJ_POWER: u32 = 0x228;
        const POWER_OFF: u32 = 0x70;
        const POWER_LEVEL: u32 = 0x1c;
        const INFO_MODE: u32 = 0x0c;
        const INFO_WANT: u32 = 4;
        const MODE_A: u32 = 0x1e;
        const MODE_B: u32 = 0x0e;
        const PED_WQB: u32 = 0x21c;
        const WQB_STATE: u32 = 0x12c;
        const WQB_WANT: u32 = 2;
        const PED_SCOPE: u32 = 0x224;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const SINGLETON: u32 = 0x0128aa90;
        const RESULT_BIT: u32 = 0x4000;
        const RESULT_OFF: u32 = 0x60;
        const OBS_KIND: u32 = 0x0c;
        const OBS_WANT: u32 = 0x19f;
        const OBS_TAG: u32 = 0x1c;
        const V_SUBJECT: u32 = 1;
        const POWER_LOOKUP: u32 = 2;
        const POWER_USE: u32 = 3;
        const WEAPON_INFO: u32 = 4;
        const SPOTTER: u32 = 5;
        const ALLOC_MAIN: u32 = 6;
        const FOLLOWUP: u32 = 7;
        const REGISTER: u32 = 8;
        const PROXIMITY: u32 = 9;
        const ALLOC_FINAL: u32 = 10;
        const BUILD_A: u32 = 11;
        const BUILD_B: u32 = 12;
        const V_OBSERVER: u32 = 13;
        const OBSERVE: u32 = 14;
        const ALT_PRED: u32 = 15;
        const ALLOC_ALT: u32 = 16;
        const ALT_A: u32 = 17;
        const ALT_B: u32 = 18;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let own = rd32(handler + HANDLER_PED);
        let subject_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(event) + EVT_SUBJECT) as usize);
        let subject = subject_of(event);
        if subject == 0 {
            return 0;
        }
        let k = rd32(event + EVT_KIND).wrapping_sub(KIND_STORE0);
        if k == 0 {
            wr32(handler + HANDLER_TASK, 0);
            return subject;
        }
        let k = k.wrapping_sub(KIND_OBSERVER - KIND_STORE0);
        if k != 0 {
            let k = k.wrapping_sub(KIND_MAIN - KIND_OBSERVER);
            if k != 0 {
                return subject;
            }
            // Main path.
            let masked = rd32(subject + SUBJ_MODE) & MODE_MASK;
            if masked != MODE_WANT {
                return masked;
            }
            if rd32(rd32(own + PED_WQB) + WQB_STATE) == WQB_WANT
                && rd8(subject + SUBJ_FLAG) != 0
            {
                let block = rd32(subject + SUBJ_POWER);
                let arg = if block == 0 { 0 } else { block + POWER_OFF };
                let power: u32 = lf_checker_rt::callee_thiscall!(POWER_LOOKUP, u32, arg);
                if power == 0 {
                    let block2 = rd32(subject + SUBJ_POWER);
                    let p = if block2 == 0 { 0 } else { block2 + POWER_OFF };
                    // Null power block reads address 0x1c: both sides fault.
                    let level = rdf(p + POWER_LEVEL);
                    if core::hint::black_box(level) > core::hint::black_box(0.0f32) {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(POWER_USE, u32, subject, 1, 0x3e8);
                        let info: u32 =
                            lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, rd32(event + EVT_WEAPON));
                        let mode = if rd32(info + INFO_MODE) == INFO_WANT {
                            MODE_A
                        } else {
                            MODE_B
                        };
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            SPOTTER,
                            u32,
                            lf_checker_rt::relocated(SINGLETON),
                            rd32(subject + SUBJ_POS) + POS_ARG,
                            mode,
                            0x3e8
                        );
                    }
                }
            }
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_MAIN, u32, pool);
            if slot != 0 {
                let slot2: u32 = lf_checker_rt::callee_thiscall!(FOLLOWUP, u32, slot, subject, 0);
                wr32(handler + HANDLER_TASK, slot2);
                wr32(slot2 + RESULT_OFF, rd32(slot2 + RESULT_OFF) | RESULT_BIT);
            } else {
                wr32(handler + HANDLER_TASK, 0);
                // Null slot: the flag update faults on address 0x60.
                wr32(RESULT_OFF, rd32(RESULT_OFF) | RESULT_BIT);
            }
            let scope = rd32(own + PED_SCOPE);
            let regged: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, scope, subject, 1);
            if rd32(handler + HANDLER_ALT) != 0 {
                return regged;
            }
            let prox: u32 = lf_checker_rt::callee_cdecl!(PROXIMITY, u32, own, event);
            if (prox as u8) == 0 {
                return prox;
            }
            let slot2: u32 = lf_checker_rt::callee_thiscall!(ALLOC_FINAL, u32, pool);
            if slot2 == 0 {
                wr32(handler + HANDLER_ALT, 0);
                return 0;
            }
            // The first build call is declared cdecl (it pops nothing) but
            // the original only drops one word after it; the second call
            // pops six words, consuming the three leftovers as its last
            // three arguments. The rewrite passes all six explicitly while
            // the stub balances each side.
            let part: u32 = lf_checker_rt::callee_stdcall!(BUILD_A, u32, own, subject, 0, 0);
            let task: u32 = lf_checker_rt::callee_thiscall!(
                BUILD_B, u32, slot2, 0, 1, part, subject, 0, 0
            );
            wr32(handler + HANDLER_ALT, task);
            return task;
        }
        // Observer path (kind 0x137).
        if observer != 0 {
            let kind_of: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(observer) + OBS_KIND) as usize);
            if kind_of(observer) == OBS_WANT && (rd16(observer + OBS_TAG) as i16) > -1 {
                let seen: u32 = lf_checker_rt::callee_thiscall!(OBSERVE, u32, observer, own);
                wr32(handler + HANDLER_TASK, 0);
                return seen;
            }
        }
        // Alternate chain.
        let pred: u32 = lf_checker_rt::callee_thiscall!(ALT_PRED, u32, own);
        if (pred as u8) != 0 {
            wr32(handler + HANDLER_TASK, 0);
            return pred;
        }
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let slot3: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ALT, u32, pool);
        if slot3 == 0 {
            wr32(handler + HANDLER_TASK, 0);
            return 0;
        }
        // Three words pushed, callee pops one; the rewrite pushes the one
        // the callee consumes.
        let g: u32 = lf_checker_rt::callee_stdcall!(ALT_A, u32, 0xbb8);
        let task: u32 = lf_checker_rt::callee_thiscall!(ALT_B, u32, slot3, 1, g);
        wr32(handler + HANDLER_TASK, task);
        task
    }
});
