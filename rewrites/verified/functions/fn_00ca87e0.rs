// original: 0x00CA87E0 CEventHandler::vf57

/// Build this handler's reaction to an event, trying a full contextual build
/// first and falling back to simpler builds when the context is missing.
///
/// `this` is the handler (its ped at `+0x04`, the built reaction stored at
/// `+0x0c`). `a1` carries a value at `+0x0c` and the event ped at `+0x10`;
/// `a3` is an object whose function-table slot `0x28` confirms the event
/// (only its low byte matters). `a2` is never read.
///
/// The full build resolves a helper from the handler ped's link at `+0x224`
/// (plus `0x44`), confirms the event through `a3`, then chains two more
/// object-table calls (slots `0x30` and `0x1c`; the last takes a scratch
/// pointer) to obtain four floats, which are copied verbatim and passed with
/// (value, event ped) to the maker along with the factory object. A missing
/// helper, a rejected event or a missing factory abandons the full build:
/// the fallback re-checks the handler ped and its link, resolves and
/// compares an identity (it must equal the event ped), optionally runs a
/// two-call gate (the second call is skipped when the first answers null,
/// and a zero low byte from the gate's second call abandons the fallback),
/// then builds through the alternate maker from (value, event ped, 1). When
/// the fallback's checks fail, the last resort builds from (value, event
/// ped) only. A missing factory at any stage stores 0. The stored reaction
/// (or 0) is also returned.
///
/// Original: 0x00CA87E0 (thiscall, ecx = handler, three stack words; the
/// second is unread. The original aligns its stack frame for the float
/// copy; the rewrite copies the four words through locals, which the
/// contract observes with a call-time snapshot).
lf_checker_rt::export!(thiscall, rw_00ca87e0(this: u32, a1: u32, _a2: u32, a3: u32) -> u32 {
    unsafe {
        const A1_VALUE: u32 = 0x0C;
        const A1_PED: u32 = 0x10;
        const H_PED: u32 = 0x04;
        const H_REACTION: u32 = 0x0C;
        const PED_LINK: u32 = 0x224;
        const HELPER_OFF: u32 = 0x44;
        const CONFIRM_SLOT: u32 = 0x28;
        const CHAIN_SLOT: u32 = 0x30;
        const FLOATS_SLOT: u32 = 0x1C;
        const G_FACTORY: u32 = 0x0167_E2A0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn factory() -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(G_FACTORY)) }
        }
        #[inline(always)]
        unsafe fn store0(this: u32) -> u32 {
            unsafe {
                wr32(this.wrapping_add(H_REACTION), 0);
                0
            }
        }

        let value = rd32(a1.wrapping_add(A1_VALUE));
        let ped = rd32(a1.wrapping_add(A1_PED));
        let hped = rd32(this.wrapping_add(H_PED));
        let link = rd32(hped.wrapping_add(PED_LINK));
        let helper: u32 =
            lf_checker_rt::callee_thiscall!(1, u32, link.wrapping_add(HELPER_OFF));
        if helper == 0 {
            return ca87e0_fallback(this, value, ped);
        }
        let confirm_vt = rd32(a3);
        let confirm: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(confirm_vt.wrapping_add(CONFIRM_SLOT)) as usize);
        if confirm(a3) & 0xFF == 0 {
            return ca87e0_fallback(this, value, ped);
        }
        let chain_vt = rd32(helper);
        let chain: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(chain_vt.wrapping_add(CHAIN_SLOT)) as usize);
        let chained = chain(helper);
        let floats_vt = rd32(chained);
        let floats: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(floats_vt.wrapping_add(FLOATS_SLOT)) as usize);
        let mut scratch = [0u32; 1];
        let got = floats(chained, scratch.as_mut_ptr() as u32);
        // Copy the four words verbatim (bit-exact for any NaN payload).
        let mut fv = [
            rd32(got),
            rd32(got.wrapping_add(4)),
            rd32(got.wrapping_add(8)),
            rd32(got.wrapping_add(12)),
        ];
        let fac: u32 = lf_checker_rt::callee_thiscall!(5, u32, factory());
        if fac == 0 {
            return store0(this);
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(
            6, u32, fac, value, ped, fv.as_mut_ptr() as u32
        );
        wr32(this.wrapping_add(H_REACTION), made);
        made
    }
});

/// Fallback build of rw_00ca87e0: re-check the ped and link, resolve the
/// identity, run the gate, build through the alternate maker.
unsafe fn ca87e0_fallback(this: u32, value: u32, ped: u32) -> u32 {
    unsafe {
        const H_PED: u32 = 0x04;
        const H_REACTION: u32 = 0x0C;
        const PED_LINK: u32 = 0x224;
        const GATE_OFF: u32 = 0x2B0;
        const G_FACTORY: u32 = 0x0167_E2A0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let hped = rd32(this.wrapping_add(H_PED));
        if hped == 0 {
            return ca87e0_lastresort(this, value, ped);
        }
        let link = rd32(hped.wrapping_add(PED_LINK));
        if link == 0 {
            return ca87e0_lastresort(this, value, ped);
        }
        let ident: u32 = lf_checker_rt::callee_thiscall!(7, u32, link);
        if ident == 0 {
            return ca87e0_lastresort(this, value, ped);
        }
        let who: u32 = lf_checker_rt::callee_thiscall!(8, u32, ident);
        if who != ped {
            return ca87e0_lastresort(this, value, ped);
        }
        let gate: u32 =
            lf_checker_rt::callee_thiscall!(9, u32, hped.wrapping_add(GATE_OFF));
        if gate != 0 {
            let gate2: u32 =
                lf_checker_rt::callee_thiscall!(9, u32, hped.wrapping_add(GATE_OFF));
            let ok: u32 = lf_checker_rt::callee_thiscall!(10, u32, gate2);
            if ok & 0xFF == 0 {
                return ca87e0_lastresort(this, value, ped);
            }
        }
        let fac: u32 = lf_checker_rt::callee_thiscall!(
            5, u32, rd32(lf_checker_rt::relocated(G_FACTORY))
        );
        if fac == 0 {
            wr32(this.wrapping_add(H_REACTION), 0);
            return 0;
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(11, u32, fac, value, ped, 1);
        wr32(this.wrapping_add(H_REACTION), made);
        made
    }
}

/// Last-resort build of rw_00ca87e0: build from (value, event ped) only.
unsafe fn ca87e0_lastresort(this: u32, value: u32, ped: u32) -> u32 {
    unsafe {
        const H_REACTION: u32 = 0x0C;
        const G_FACTORY: u32 = 0x0167_E2A0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let fac: u32 = lf_checker_rt::callee_thiscall!(
            5, u32, rd32(lf_checker_rt::relocated(G_FACTORY))
        );
        if fac == 0 {
            wr32(this.wrapping_add(H_REACTION), 0);
            return 0;
        }
        let made: u32 = lf_checker_rt::callee_thiscall!(12, u32, fac, value, ped);
        wr32(this.wrapping_add(H_REACTION), made);
        made
    }
}
