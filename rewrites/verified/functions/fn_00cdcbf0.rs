// original: 0x00CDCBF0 nm_task_timer_update (proposed)

/// Decrement this task's two NaturalMotion retrigger timers by the frame
/// step and send a NaturalMotion message when one of them expires.
///
/// `this` is the task object. Its flag word at `+0x60` selects the work:
/// bits 9-10 (`NEED_MASK`) say the first timer path is live, bit 6
/// (`HOLD_BIT`) skips the ped-state gate. The ped pointer `ped` carries two
/// state bytes at `+0x218`/`+0x219` and the NaturalMotion context at `+0x7b4`.
///
/// First-timer path (flag bits 9-10 set): when the timer at `+0x78` is above
/// zero it is reduced by the frame step (global `DT_STEP`) and stored back;
/// a result at or below zero fires. Otherwise the ped gate runs: the first
/// state byte must be clear while the second is set, the ped-state callee
/// must return non-null, and two bytes of its answer, each xored with a
/// third, must land on opposite sides of 0x7f. Firing sends one message:
/// build a message on the stack (constructor, one boolean slot, send through
/// the ped's context, destructor) and clear the flag word.
///
/// When the flag bits are clear only the second timer (at `+0x74`) runs: it
/// is reduced the same way and fires (same message shape, flag word cleared)
/// when the result reaches zero or below. A timer at or below zero, or NaN,
/// never fires.
///
/// Returns whatever the last callee returned, or a fixed shuffle of the flag
/// word when no callee ran. Original: 0x00CDCBF0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00cdcbf0(this: u32, ped: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x60;
        const TIMER_A: u32 = 0x78;
        const TIMER_B: u32 = 0x74;
        const PED_STATE0: u32 = 0x218;
        const PED_STATE1: u32 = 0x219;
        const PED_NMCTX: u32 = 0x7b4;
        const NEED_MASK: u32 = 0x600;
        const HOLD_BIT: u32 = 0x40;
        const KEY_BASE: u32 = 0x26cc;
        const KEY_MID: u32 = 0x26ce;
        const KEY_TOP: u32 = 0x26cf;
        const DT_STEP: u32 = 0x11735bc;
        const MSG_NAME_BOOL: u32 = 0x1051cc8;
        const MSG_NAME_SEND: u32 = 0x1051f44;
        const PED_STATE_CALLEE: u32 = 1;
        const NM_CTOR: u32 = 2;
        const NM_SET_BOOL: u32 = 3;
        const NM_SEND: u32 = 4;
        const NM_DTOR: u32 = 5;
        const COOKIE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        /// Build one message on a stack buffer and send it through `ctx`.
        /// Returns the last callee's answer, like the original's eax.
        unsafe fn send_once(ctx: u32) -> u32 {
            unsafe {
                let mut msg = [0u32; 16];
                let buf = msg.as_mut_ptr() as u32;
                let mut r = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf);
                let name_bool = rd32(lf_checker_rt::relocated(MSG_NAME_BOOL));
                r = lf_checker_rt::callee_thiscall!(NM_SET_BOOL, u32, buf, name_bool, 0);
                let name_send = rd32(lf_checker_rt::relocated(MSG_NAME_SEND));
                r = lf_checker_rt::callee_thiscall!(NM_SEND, u32, ctx, name_send, buf);
                r = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf);
                r
            }
        }

        let flags = rd32(this + FLAGS);
        let live = (flags & NEED_MASK) != 0;
        let hold = (flags & HOLD_BIT) != 0;
        // Value the original's shr/and pair leaves in eax when no call runs.
        let mut eax = ((flags >> 6) & 0xFFFF_FF00) | ((flags >> 6) & 1);
        let dt = f32::from_bits(rd32(lf_checker_rt::relocated(DT_STEP)));

        // First-timer path and ped gate (the original's L43/L81).
        let mut fire_a = false;
        if live {
            let t = rdf(this + TIMER_A);
            if t > 0.0 {
                let rest = sub(t, dt);
                wrf(this + TIMER_A, rest);
                if 0.0 >= rest {
                    fire_a = true;
                }
            }
        }
        if !fire_a {
            if hold {
                fire_a = true;
            } else if rd8(ped + PED_STATE0) == 0 && rd8(ped + PED_STATE1) != 0 {
                let r = lf_checker_rt::callee_thiscall!(PED_STATE_CALLEE, u32, ped);
                eax = r;
                if r != 0 {
                    let dl = rd8(r + KEY_BASE);
                    let mid = rd8(r + KEY_MID) ^ dl;
                    if mid > 0x7f {
                        let top = rd8(r + KEY_TOP) ^ dl;
                        if top <= 0x7f {
                            fire_a = true;
                        }
                    }
                }
            }
        }
        if fire_a && live {
            eax = send_once(rd32(ped + PED_NMCTX));
            wr32(this + FLAGS, 0);
        }

        // Second timer runs only when the flag bits are clear.
        if !live {
            let t = rdf(this + TIMER_B);
            if t > 0.0 {
                let rest = sub(t, dt);
                wrf(this + TIMER_B, rest);
                if rest <= 0.0 {
                    eax = send_once(rd32(ped + PED_NMCTX));
                    wr32(this + FLAGS, 0);
                }
            }
        }

        lf_checker_rt::callee_stdcall!(COOKIE, u32,);
        eax
    }
});

