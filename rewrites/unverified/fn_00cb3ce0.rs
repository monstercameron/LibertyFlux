// original: 0x00cb3ce0 CTaskComplexMoveGoToShelterAndWait::vf18

/// Pick the go-to-shelter subtask from the child task type and wait state.
///
/// `this` is the complex task, `ped` the pedestrian. The active child type
/// comes from virtual slot 3 of the sub-object at `+0x08`; the wait state
/// lives at `+0x2c`, the shelter handle at `+0x30`, a flag byte at `+0x3c`,
/// and a position buffer at `+0x40`.
///
/// Non-wait children map straight to a request: `0x386` also records state
/// 2 and requests `0x11a`; `0x3ae` requests `0x386`; anything else requests
/// `0x516`. For a waiting child (`0x11a`), state 0 (or any state other than
/// 2) with no shelter requests `0x516`; with a shelter it validates the
/// handle (callee 3): invalid requests `0x11a`, valid resolves it through
/// callee 4, clears the handle, then requests `0x516` on success or records
/// state 1 and requests `0x3ae` on failure. State 2 with the flag clear
/// polls the timer callee (id 5) and compares a global clock against 0.2:
/// a below-or-unordered clock, or an elapsed timer, requests `0x516`,
/// otherwise `0x11a`. State 2 with the flag set asks callee 6 and requests
/// `0x516` when it agrees, else `0x11a`.
///
/// The clock comparison takes the below branch on unordered (NaN) too, so
/// it is written as `!(g >= 0.2)`. Callee answers in `al` are masked; their
/// upper bits are residue.
///
/// Original: 0x00cb3ce0 (thiscall, one stack word; returns the subtask id).
lf_checker_rt::export!(thiscall, rw_00cb3ce0(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x08;
        const VTABLE_SLOT: u32 = 0x0c;
        const STATE: u32 = 0x2c;
        const SHELTER: u32 = 0x30;
        const CHECK_CTX: u32 = 0x34;
        const FLAG: u32 = 0x3c;
        const POS_BUF: u32 = 0x40;
        const WAIT_TYPE: u32 = 0x11a;
        const SHELTER_TYPE: u32 = 0x386;
        const GOTO_TYPE: u32 = 0x3ae;
        const IDLE_TYPE: u32 = 0x516;
        const CLOCK_FILE_VA: u32 = 0x012ddeac;
        const CHECK_CALLEE: u32 = 3;
        const RESOLVE_CALLEE: u32 = 4;
        const TIMER_CALLEE: u32 = 5;
        const CONFIRM_CALLEE: u32 = 6;
        const REQUEST_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let sub = rd32(this + SUBTASK);
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(sub) + VTABLE_SLOT) as usize);
        let child = slot(sub);
        if child != WAIT_TYPE {
            if child == SHELTER_TYPE {
                ((this + STATE) as *mut u32).write(2);
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, WAIT_TYPE, ped);
            }
            if child == GOTO_TYPE {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, SHELTER_TYPE, ped);
            }
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, IDLE_TYPE, ped);
        }
        let state = rd32(this + STATE);
        if state == 2 {
            if ((this + FLAG) as *const u8).read() != 0 {
                let agree =
                    (lf_checker_rt::callee_thiscall!(CONFIRM_CALLEE, u32, this + CHECK_CTX) & 0xff)
                        as u8;
                if agree != 0 {
                    return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, IDLE_TYPE, ped);
                }
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, WAIT_TYPE, ped);
            }
            let done =
                (lf_checker_rt::callee_cdecl!(TIMER_CALLEE, u32, ped) & 0xff) as u8;
            let clock = f32::from_bits(rd32(lf_checker_rt::relocated(CLOCK_FILE_VA)));
            if !(clock >= 0.2f32) {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, IDLE_TYPE, ped);
            }
            if done != 0 {
                return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, IDLE_TYPE, ped);
            }
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, WAIT_TYPE, ped);
        }
        let shelter = rd32(this + SHELTER);
        if shelter == 0 {
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, IDLE_TYPE, ped);
        }
        let valid = (lf_checker_rt::callee_cdecl!(CHECK_CALLEE, u32, shelter) & 0xff) as u8;
        if valid == 0 {
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, WAIT_TYPE, ped);
        }
        let resolved: u32 = lf_checker_rt::callee_cdecl!(RESOLVE_CALLEE, u32, shelter, this + POS_BUF);
        ((this + SHELTER) as *mut u32).write(0);
        if resolved == 0 {
            ((this + STATE) as *mut u32).write(1);
            return lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, GOTO_TYPE, ped);
        }
        lf_checker_rt::callee_thiscall!(REQUEST_CALLEE, u32, this, IDLE_TYPE, ped)
    }
});
