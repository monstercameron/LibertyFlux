// original: 0x00a6e140 task_request_with_followup (proposed)

/// Builds a follow-up request for the task `arg1` and submits it, then
/// optionally triggers the result's follow-up hook.
///
/// First computes a sticky flag: set exactly when the back-link at
/// `arg2 + BACKLINK` points at `arg1`, is non-null, carries the `WANT_BITS`
/// masked bits at `+ CLASS_BITS`, and has state `READY_STATE` at
/// `+ LINK_STATE`. (The original stashes this flag in its incoming `arg2`
/// slot; the rewrite keeps it in a local, which is why this contract runs
/// with the stack check off.)
///
/// Returns null when the request byte (low byte of `arg0`) is zero or
/// `arg1` is null. Otherwise seeds two accumulators: when the state word at
/// `arg1 + STATE` equals `FULL_STATE`, the option byte at `arg1 + OPT` must
/// have bit 1 or the result is null, and the first accumulator becomes
/// `OPT_ALT` or `OPT_BASE` depending on bit 4 of the next byte. The gate
/// routine (callee 1) runs; on a non-zero low byte and a non-null word at
/// `arg1 + EXTRA`, the second accumulator drops from `CODE_A` to `CODE_B`.
/// A `FULL_STATE` state then forces the second accumulator to `CODE_C`,
/// else the first gains `FLAG_BIT`.
///
/// The manager is fetched (callee 2); a null manager yields null. Otherwise
/// the submit routine (callee 3) runs with the manager in ecx and the words
/// `(arg1, accumulator2, SUBMIT_TAG, accumulator1, 0)`, and its result is
/// kept. When the sticky flag is set, the result's follow-up virtual at
/// slot `FOLLOWUP_SLOT` runs (callee 4, through the result's vtable, the
/// result in ecx, word `FOLLOWUP_ARG`). Returns the submit result.
///
/// Original: stdcall, three stack words (`arg0`, `arg1`, `arg2`),
/// callee pops 12.
lf_checker_rt::export!(stdcall, rw_00a6e140(arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const BACKLINK: u32 = 0xab0;
        const CLASS_BITS: u32 = 0x28;
        const WANT_BITS: u32 = 0x3c0;
        const WANT_CLASS: u32 = 0x80;
        const LINK_STATE: u32 = 0x1304;
        const READY_STATE: u32 = 2;
        const STATE: u32 = 0x1304;
        const FULL_STATE: u32 = 3;
        const OPT: u32 = 0x14e4;
        const OPT_BASE: u32 = 0x20000;
        const OPT_ALT: u32 = 0x40000;
        const EXTRA: u32 = 0xf50;
        const CODE_A: u32 = 0xffff_fff9;
        const CODE_B: u32 = 0xffff_fffb;
        const CODE_C: u32 = 0xffff_fffa;
        const FLAG_BIT: u32 = 0x200_0000;
        const SUBMIT_TAG: u32 = 0x1b;
        const FOLLOWUP_SLOT: u32 = 0x58;
        const FOLLOWUP_ARG: u32 = 9;
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const GATE: u32 = 1;
        const GET_MANAGER: u32 = 2;
        const SUBMIT: u32 = 3;

        let backlink =
            ((arg2 as *const u32).wrapping_byte_offset(BACKLINK as isize)).read_unaligned();
        let sticky = backlink == arg1
            && backlink != 0
            && (((backlink as *const u32).wrapping_byte_offset(CLASS_BITS as isize))
                .read_unaligned()
                & WANT_BITS)
                == WANT_CLASS
            && (((backlink as *const u32).wrapping_byte_offset(LINK_STATE as isize))
                .read_unaligned())
                == READY_STATE;
        if (arg0 as u8) == 0 || arg1 == 0 {
            return 0;
        }
        let mut acc1 = 0u32;
        let state =
            ((arg1 as *const u32).wrapping_byte_offset(STATE as isize)).read_unaligned();
        if state == FULL_STATE {
            let opt = ((arg1 as *const u8).wrapping_byte_offset(OPT as isize)).read();
            if opt & 2 == 0 {
                return 0;
            }
            let alt = ((arg1 as *const u8).wrapping_byte_offset((OPT + 1) as isize)).read();
            acc1 = if alt & 0x10 != 0 { OPT_ALT } else { OPT_BASE };
        }
        let mut acc2 = CODE_A;
        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32,);
        if (gate as u8) != 0 {
            let extra =
                ((arg1 as *const u32).wrapping_byte_offset(EXTRA as isize)).read_unaligned();
            if extra != 0 {
                acc2 = CODE_B;
            }
        }
        if state == FULL_STATE {
            acc2 = CODE_C;
        } else {
            acc1 |= FLAG_BIT;
        }
        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        let out: u32 =
            lf_checker_rt::callee_thiscall!(SUBMIT, u32, mgr, arg1, acc2, SUBMIT_TAG, acc1, 0);
        if sticky {
            let vtable = (out as *const u32).read_unaligned();
            let target = ((vtable as *const u32).wrapping_byte_offset(FOLLOWUP_SLOT as isize))
                .read_unaligned();
            let followup: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            followup(out, FOLLOWUP_ARG);
        }
        out
    }
});
