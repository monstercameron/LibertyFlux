// original: 0x00b77fe0 task_find_or_attach (proposed)

/// Find the task matching `key` or attach `ptr` to a fresh one, returning
/// `key` on success and 0xff when nothing accepts it.
///
/// The search runs in stages. First the probe callee is asked about `ptr`:
/// a null answer skips ahead, a task whose key field (`+0x1a0`) equals `key`
/// returns at once, and any other task is detached from `ptr` first. Then
/// the lookup callee is asked about `key`: a task in state (`+0x19c`) 1 or
/// 2 is offered `ptr` through the attach callee with the (`flag` == 7) bit
/// and success returns `key`, anything else returns 0xff. Then the scan
/// callee is asked about `flag`: a task found there gets `ptr` attached with
/// a zero bit plus `key` recorded, and success returns `key`. Then the
/// fallback callee is asked about `flag`: a task found there is offered
/// (`ptr`, `key`) and success returns `key`, failure 0xff.
///
/// If every stage misses and `flag` is not 2, the answer is 0xff. Otherwise
/// a bounded random draw seeds a three-step sweep: step `i` looks up
/// (`draw` + `i`) mod 3 in the table callee, and a task in state 1 or 2 is
/// offered (`ptr`, RETADDR) through the attach callee, where RETADDR is the
/// word above this function's own frame, i.e. its return address. On
/// success the task's key field is returned.
///
/// About RETADDR: the original reads its caller's return address and passes
/// it as the attach callee's second argument. That callee only tests the
/// argument's low byte for zero (verified by reading its body: three loads,
/// two zero-tests, no other use), and all four known direct callers of this
/// function return to addresses with a nonzero low byte, so in the shipped
/// game the callee always takes its nonzero path here. A Rust rewrite cannot
/// read its own return address, so this rewrite passes 1, which takes the
/// same callee path under the same precondition. The contract skips that
/// one argument (it cannot match: each side would pass its own address) and
/// compares everything else, including every other argument of every other
/// attach call. If a future caller returned to an address with a zero low
/// byte, the original would take the callee's zero path there while this
/// rewrite would not.
///
/// The (flag == 7) bit travels in the low byte of a stack dword whose upper
/// bytes the original never writes; the contract fixes the stack fill to
/// zero so the whole word compares, and the callee ignores those bytes
/// anyway (low-byte test only).
///
/// Original: 0x00b77fe0 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00b77fe0(key: u32, ptr: u32, flag: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x1a0;
        const STATE_OFF: u32 = 0x19c;
        const MISS: u32 = 0xff;
        const FLAG_SEVEN: u32 = 7;
        const FLAG_SWEEP: u32 = 2;
        const SWEEP_STEPS: u32 = 3;
        // Stand-in for the original's return address (see doc comment).
        const RETADDR_STANDIN: u32 = 1;
        const PROBE_CALLEE: u32 = 1;
        const DETACH_CALLEE: u32 = 2;
        const LOOKUP_CALLEE: u32 = 3;
        const ATTACH_U_CALLEE: u32 = 4;
        const ATTACH_V_CALLEE: u32 = 5;
        const ATTACH_SWEEP_CALLEE: u32 = 6;
        const SCAN_CALLEE: u32 = 7;
        const RECORD_CALLEE: u32 = 8;
        const FALLBACK_CALLEE: u32 = 9;
        const OFFER_CALLEE: u32 = 10;
        const DRAW_CALLEE: u32 = 11;
        const TABLE_CALLEE: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn attachable(state: u32) -> bool {
            state == 1 || state == 2
        }

        let t: u32 = lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, ptr);
        if t != 0 {
            if rd32(t.wrapping_add(KEY_OFF)) == key {
                return key;
            }
            let _: u32 = lf_checker_rt::callee_thiscall!(DETACH_CALLEE, u32, t, ptr);
        }

        let seven = (flag == FLAG_SEVEN) as u32;
        let u: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, key);
        if u != 0 {
            if attachable(rd32(u.wrapping_add(STATE_OFF))) {
                let ok: u32 =
                    lf_checker_rt::callee_thiscall!(ATTACH_U_CALLEE, u32, u, ptr, seven);
                if ok & 0xff != 0 {
                    return key;
                }
            }
            return MISS;
        }

        let v: u32 = lf_checker_rt::callee_cdecl!(SCAN_CALLEE, u32, flag);
        if v != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(ATTACH_V_CALLEE, u32, v, ptr, 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(RECORD_CALLEE, u32, v, key);
            return key;
        }

        let w: u32 = lf_checker_rt::callee_cdecl!(FALLBACK_CALLEE, u32, flag);
        if w != 0 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(OFFER_CALLEE, u32, w, ptr, key);
            if ok & 0xff != 0 {
                return key;
            }
            return MISS;
        }

        if flag != FLAG_SWEEP {
            return MISS;
        }
        let draw: u32 = lf_checker_rt::callee_cdecl!(DRAW_CALLEE, u32, 0, flag);
        let mut i = 0u32;
        while i < SWEEP_STEPS {
            let d = draw.wrapping_add(i) % SWEEP_STEPS;
            let o: u32 = lf_checker_rt::callee_cdecl!(TABLE_CALLEE, u32, d);
            if attachable(rd32(o.wrapping_add(STATE_OFF))) {
                let ok: u32 = lf_checker_rt::callee_thiscall!(
                    ATTACH_SWEEP_CALLEE,
                    u32,
                    o,
                    ptr,
                    RETADDR_STANDIN
                );
                if ok & 0xff != 0 {
                    return rd32(o.wrapping_add(KEY_OFF));
                }
            }
            i += 1;
        }
        MISS
    }
});
