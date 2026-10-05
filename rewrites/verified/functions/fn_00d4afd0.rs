// original: 0x00D4AFD0 CTaskSimpleHandsUp::vf17

#![allow(unsafe_code)]

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    unsafe { (a as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

/// Hands-up task update (`vf17`): run one tick for the ped.
///
/// `this` is the task, `ped` the ped. An already-finished task (bit 0 at
/// `+0x18`) returns 1 at once. Otherwise, when the timer at `+0x38` is
/// armed and the deadline `+0x34` has passed the current time, the weapon
/// state is queried: a positive answer whose flag word carries bit `0x40`
/// clears the aim flag at bit 6 of the target's word and returns 0. If the
/// start bit at `+0xc` is clear the task's virtual start hook (slot
/// `+0x14`) runs with the ped, setting bit 1 of `+0xc` on success. With no
/// live target the animation is resolved from the ped's anim state; when
/// that also finds nothing the timer is re-armed from `+0x2c` and the
/// restart helper runs. Returns bit 0 of `+0x18` (1 when finished).
///
/// Original: 0x00D4AFD0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00D4AFD0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TIMER: u32 = 0x011735B4;
        const GUNSTATE: u32 = 0x016DD63C;
        const COOKIE: u32 = 0x01057FB4;
        const QUERY: u32 = 0;
        const START_HOOK: u32 = 1;
        const ANIM_FIND: u32 = 2;
        const RESTART: u32 = 3;
        const COOKIE_CHECK: u32 = 4;

        #[inline(always)]
        unsafe fn timer() -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(TIMER) as *const u32).read_unaligned() }
        }
        // The original checks its stack cookie before every return; the
        // stub preserves all registers, so this only records the call.
        #[inline(always)]
        unsafe fn cookie() {
            unsafe {
                let c: u32 = (lf_checker_rt::global::<u32>(COOKIE) as *const u32).read_unaligned();
                let _: u32 = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, c);
            }
        }

        if rd8(this + 0x18) & 1 != 0 {
            cookie();
            return 1;
        }
        if rd8(this + 0x38) != 0 {
            if rd8(this + 0x39) != 0 {
                wr32(this + 0x30, timer());
                wr8(this + 0x39, 0);
            }
            let now = timer();
            let end = rd32(this + 0x34).wrapping_add(rd32(this + 0x30));
            if (end as i32) <= (now as i32) {
                let state: u32 = (lf_checker_rt::global::<u32>(GUNSTATE) as *const u32).read_unaligned();
                // The query fills two words; the original tests bit 0x40 of
                // the second word's low byte (the byte past the first word).
                let mut flags = [0u32; 2];
                let ok: u32 = lf_checker_rt::callee_thiscall!(QUERY, u32, state,
                    rd32(this + 0x1c), rd32(this + 0x20),
                    (&mut flags as *mut u32) as u32);
                if ok as u8 != 0 && flags[1] & 0x40 != 0 {
                    let target = rd32(this + 0x14);
                    if target != 0 {
                        let w = rd32(target + 4);
                        if (w >> 6) & 1 != 0 {
                            wr32(target + 4, w & !0x40);
                        }
                        cookie();
                        return 0;
                    }
                }
                if rd8(this + 0x0c) & 1 == 0 {
                    let slot = rd32(rd32(this) + 0x14);
                    let start: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    if start(this, ped, 0, 0) as u8 != 0 {
                        wr32(this + 0x0c, rd32(this + 0x0c) | 2);
                    }
                }
            }
        }
        if rd32(this + 0x14) != 0 {
            cookie();
            return (rd8(this + 0x18) & 1) as u32;
        }
        let found: u32 = lf_checker_rt::callee_thiscall!(ANIM_FIND, u32, rd32(ped + 0x78),
            rd32(this + 0x1c), rd32(this + 0x20));
        if found != 0 {
            cookie();
            return (rd8(this + 0x18) & 1) as u32;
        }
        wr32(this + 0x34, rd32(this + 0x2c));
        wr32(this + 0x30, timer());
        wr8(this + 0x38, 1);
        let _: u32 = lf_checker_rt::callee_thiscall!(RESTART, u32, this, ped);
        cookie();
        (rd8(this + 0x18) & 1) as u32
    }
});
