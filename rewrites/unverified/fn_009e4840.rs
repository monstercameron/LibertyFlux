// original: 0x009E4840 ped_task_run_gate (proposed)

/// Decide whether a ped task may run, through a gauntlet of state flags and
/// helper calls, answering 1 for go and 0 for stop (low byte; the upper
/// three bytes are the last value `eax` held, low byte replaced).
///
/// `this` is the task object, `arg` a caller flag whose low byte must be
/// non-zero on one path. A manager object from a writable global is asked
/// first: an answer below 1 (signed) takes an early reporting path that
/// always answers 0. Otherwise two virtual gates (slots `+0x34` and `+0xd4`,
/// the second called with 1.0 and its answer converted by a helper) and an
/// optional state check must all pass, then four flag bytes must be clear,
/// a resource lookup must answer null or carry a kind other than `0x80`,
/// and a mode word must agree with a global mode. The tail runs one final
/// helper with a scratch out-word and answers whether its low byte is
/// non-zero. Every exit also performs the compiler's stack-cookie check,
/// modelled as a register-preserving call.
///
/// Original: 0x009E4840 (thiscall, this + one stack word).
lf_checker_rt::export!(thiscall, rw_009E4840(this: u32, arg: u32) -> u32 {
    unsafe {
        const MGR_GLOB: u32 = 0x018B6F10;
        const MODE_GLOB: u32 = 0x011D6FD4;
        const LOG_STR: u32 = 0x00E98484;
        const FIXED_OBJ: u32 = 0x01908EF0;
        const VT_GATE: u32 = 0x34;
        const VT_OBJ: u32 = 0xD4;
        const C_MGR: u32 = 1;
        const C_REP: u32 = 2;
        const C_LOG: u32 = 3;
        const C_COOKIE: u32 = 4;
        const C_VGATE: u32 = 5;
        const C_VOBJ: u32 = 6;
        const C_CONV: u32 = 7;
        const C_CHK: u32 = 8;
        const C_RES: u32 = 9;
        const C_F: u32 = 10;
        const C_H: u32 = 11;
        const C_I: u32 = 12;
        const C_M: u32 = 13;
        const C_J: u32 = 14;
        const C_K: u32 = 15;
        const C_L: u32 = 16;
        const C_TAIL: u32 = 17;
        const LOW_MASK: u32 = 0xFFFF_FF00;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// Failure exit: the cookie check, then `eax` with its low byte cleared.
        #[inline(always)]
        unsafe fn fail(eax: u32) -> u32 {
            unsafe {
                lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
                eax & LOW_MASK
            }
        }
        /// Tail path: one final helper, then whether its low byte is non-zero.
        unsafe fn tail(this: u32) -> u32 {
            unsafe {
                let mut slot = 0u32;
                let sp = &mut slot as *mut u32 as u32;
                let t: u32 = lf_checker_rt::callee_thiscall!(C_TAIL, u32, this, sp);
                lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
                (t & LOW_MASK) | u32::from(t & 0xFF != 0)
            }
        }

        let mgr = rd32(lf_checker_rt::relocated(MGR_GLOB));
        let mut eax: u32 = lf_checker_rt::callee_thiscall!(C_MGR, u32, mgr);
        if (eax as i32) < 1 {
            let sub = rd32(this.wrapping_add(0x6C));
            if sub == 0 {
                return fail(eax);
            }
            if rd8(sub.wrapping_add(0x0E)) == 0 {
                return fail(eax);
            }
            eax = lf_checker_rt::callee_thiscall!(C_REP, u32, sub);
            let mut scratch = 0u32;
            let sp = &mut scratch as *mut u32 as u32;
            eax = lf_checker_rt::callee_cdecl!(C_LOG, u32, sp, LOG_STR, eax);
            return fail(eax);
        }
        let sub = rd32(this.wrapping_add(0x6C));
        if sub == 0 || rd8(sub.wrapping_add(0x0E)) == 0 {
            let vt = rd32(this);
            let gate: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt.wrapping_add(VT_GATE)) as usize) };
            eax = gate(this);
            if eax & 0xFF == 0 {
                return fail(eax);
            }
            let vt2 = rd32(this);
            let get: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(vt2.wrapping_add(VT_OBJ)) as usize) };
            eax = get(this, 0x3F800000);
            eax = lf_checker_rt::callee_thiscall!(C_CONV, u32, eax);
            if eax & 0xFF == 0 {
                return fail(eax);
            }
            if rd8(this.wrapping_add(0x211)) != 0 {
                eax = lf_checker_rt::callee_thiscall!(C_CHK, u32, this);
                if eax & 0xFF == 0 {
                    return fail(eax);
                }
            }
        }
        if rd8(this.wrapping_add(0x219)) != 0 {
            return fail(eax);
        }
        if rd8(this.wrapping_add(0x26C)) & 4 != 0 {
            return fail(eax);
        }
        if rd8(this.wrapping_add(0x29C)) & 4 != 0 {
            return fail(eax);
        }
        if rd8(this.wrapping_add(0x118)) & 1 != 0 {
            return fail(eax);
        }
        eax = lf_checker_rt::callee_thiscall!(C_RES, u32, this);
        if eax != 0 {
            eax = lf_checker_rt::callee_thiscall!(C_RES, u32, this);
            eax = rd32(eax.wrapping_add(0x28)) & 0x3C0;
            if eax == 0x80 {
                return fail(eax);
            }
        }
        let flags = rd32(this.wrapping_add(0x260));
        if flags & 0x80000 == 0 && flags & 0x60000 != 0 {
            return fail(eax);
        }
        eax = lf_checker_rt::callee_thiscall!(C_F, u32, this);
        if eax & 0xFF != 0 {
            return fail(eax);
        }
        let mode = rd32(lf_checker_rt::relocated(MODE_GLOB));
        let feat = rd32(this.wrapping_add(0x264));
        let ok: u32;
        if mode == 2 && feat & 0x4000000 != 0 {
            eax = (eax & LOW_MASK) | 1;
            ok = 1;
        } else {
            eax = eax & LOW_MASK;
            ok = 0;
        }
        if flags & 0x8000 != 0 && ok == 0 {
            return fail(eax);
        }
        if rd32(this.wrapping_add(0x6C)) == 0 {
            eax = lf_checker_rt::callee_cdecl!(C_H, u32,);
            if eax & 0xFF == 0 {
                return tail(this);
            }
        }
        eax = lf_checker_rt::callee_thiscall!(C_I, u32, this);
        if eax & 0xFF == 0 {
            eax = u32::from(rd8(this.wrapping_add(0xA60)) == 2);
            eax = lf_checker_rt::callee_thiscall!(C_J, u32, FIXED_OBJ, 2u32, eax);
            if eax & 0xFF == 0 {
                return fail(eax);
            }
            let sub2 = rd32(this.wrapping_add(0x6C));
            if sub2 == 0 {
                return tail(this);
            }
            eax = lf_checker_rt::callee_thiscall!(C_K, u32, sub2);
            if eax & 0xFF != 0 {
                return fail(eax);
            }
            eax = lf_checker_rt::callee_thiscall!(C_L, u32, sub2, 0x20u32);
            if eax & 0xFF != 0 {
                return fail(eax);
            }
            return tail(this);
        }
        if arg & 0xFF == 0 {
            return fail(eax);
        }
        eax = u32::from(rd8(this.wrapping_add(0xA60)) == 2);
        eax = lf_checker_rt::callee_cdecl!(C_M, u32, 2u32, eax, 0u32);
        if eax & 0xFF != 0 {
            return tail(this);
        }
        fail(eax)
    }
});
