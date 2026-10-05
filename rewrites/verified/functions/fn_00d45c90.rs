// original: 0x00D45C90 CTaskSimplePlayRandomAmbients::vf5

/// Advance the play-random-ambients task one step, serving its state machine.
///
/// `this` is the task (`+0x78` state, `+0x24`/`+0x28` live effect handles,
/// `+0x1c` a saved word). `ped` (first stack word) is the ped the task runs
/// on; the middle stack word is never read; `event` (third stack word) may be
/// null and is queried through its virtual slot at `+4` when present.
///
/// Behaviour, in order: when the state is 5, mark the ped (`+0x368`) as
/// finishing (3, or 2 when the event answers `0x20`) and clear the state.
/// Release each live effect handle (release helper, then a settle helper with
/// `-4.0`, then clear the slot). When the state is 4 and the event answers
/// `0x78` (or answers `0x7f` on a second query), look at the ped's ambient
/// block (`+0x2b0`): when it yields an entry of kind `0x2e`, request that
/// entry and clear the state. Always run the ped's `+0x3c0` step helper.
/// Finally, when the event answers `0x83` and the state is 2, run the task's
/// own step helper preserving `+0x1c` and re-stating 2; otherwise run the step
/// helper plainly. Returns with the low byte set to 1 over the step helper's
/// upper return bytes, exactly like the original's `(an instruction of the original)`.
///
/// Original: 0x00D45C90 (thiscall, three stack words, returns u32).
lf_checker_rt::export!(thiscall, rw_00D45C90(this: u32, ped: u32, _unused: u32, event: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x78;
        const HANDLE_A: u32 = 0x24;
        const HANDLE_B: u32 = 0x28;
        const SAVED: u32 = 0x1c;
        const PED_MARK: u32 = 0x368;
        const PED_AMBIENT: u32 = 0x2b0;
        const PED_STEP: u32 = 0x3c0;
        const SETTLE: u32 = 0xC080_0000;
        const WANT_KIND: u32 = 0x2e;
        const EVT_CALLEE: u32 = 1;
        const RELEASE_CALLEE: u32 = 2;
        const SETTLE_CALLEE: u32 = 3;
        const LOOKUP_CALLEE: u32 = 4;
        const REQUEST_CALLEE: u32 = 5;
        const PEDSTEP_CALLEE: u32 = 6;
        const TASKSTEP_CALLEE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn query_event(event: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(event).wrapping_add(4));
                let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
                f(event)
            }
        }
        #[inline(always)]
        unsafe fn release_handle(obj: u32, this: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, obj, this);
                lf_checker_rt::callee_thiscall!(SETTLE_CALLEE, u32, obj, SETTLE);
            }
        }

        if rd32(this.wrapping_add(STATE)) == 5 {
            wr32(ped.wrapping_add(PED_MARK), 3);
            if event != 0 && query_event(event) == 0x20 {
                wr32(ped.wrapping_add(PED_MARK), 2);
            }
            wr32(this.wrapping_add(STATE), 0);
        }
        let ha = rd32(this.wrapping_add(HANDLE_A));
        if ha != 0 {
            release_handle(ha, this);
            wr32(this.wrapping_add(HANDLE_A), 0);
        }
        let hb = rd32(this.wrapping_add(HANDLE_B));
        if hb != 0 {
            release_handle(hb, this);
            wr32(this.wrapping_add(HANDLE_B), 0);
        }
        if rd32(this.wrapping_add(STATE)) == 4 && event != 0 {
            let first = query_event(event);
            let take = first == 0x78 || (first != 0x78 && query_event(event) == 0x7f);
            if take {
                let block = ped.wrapping_add(PED_AMBIENT);
                let got = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, block);
                if got != 0 {
                    let entry = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, block);
                    if rd32(entry.wrapping_add(0x18)) == WANT_KIND {
                        lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, block, ped, WANT_KIND, 1);
                    }
                }
                wr32(this.wrapping_add(STATE), 0);
            }
        }
        lf_checker_rt::callee_thiscall!(PEDSTEP_CALLEE, u32, ped.wrapping_add(PED_STEP));
        let r = if event != 0 && query_event(event) == 0x83 && rd32(this.wrapping_add(STATE)) == 2 {
            let saved = rd32(this.wrapping_add(SAVED));
            let r = lf_checker_rt::callee_thiscall!(TASKSTEP_CALLEE, u32, this, ped);
            wr32(this.wrapping_add(SAVED), saved);
            wr32(this.wrapping_add(STATE), 2);
            r
        } else {
            lf_checker_rt::callee_thiscall!(TASKSTEP_CALLEE, u32, this, ped)
        };
        (r & 0xFFFF_FF00) | 1
    }
});
