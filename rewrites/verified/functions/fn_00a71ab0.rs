// original: 0x00A71AB0 build_player_task_by_id (proposed)

/// Build a player task for a task id: a small factory switch.
///
/// `this` is the owning object, `obj` a game object, `id` the task id.
/// Unknown ids return null. The six known ids, with the pool allocator
/// behind a data global supplying every block (a null block returns null,
/// except where noted):
///
/// * `0xD3`: clear bit 29 of `obj + 0x270`; build a jump task (its own
///   block may be null, contributing null) and a helper on a third block
///   (a null third block contributes zero); attach both to the first
///   block (thiscall: helper-or-zero, jump-or-null, 1, 0) and return the
///   attach result.
/// * `8`: zero `this + 0x70` and return a freshly built idles task.
/// * `6`: return a freshly built gun task.
/// * `0x11F`: return a freshly built climb task (argument 0).
/// * `0x1AF`: pick `obj + 0x398` when it is non-null and its record flags
///   at `+0x28` masked with `0x3C0` equal `0xC0` (else null); return a
///   freshly built melee task (thiscall: 0, 0, 0, 0, 1, pick, 1).
/// * `0x146`: like `0xD3` but with a plain task and its own helper, then
///   the same attach.
///
/// The big-id test is a signed greater-than against `0x11F`, so negative
/// ids always take the default null path.
///
/// Original: 0x00A71AB0 (thiscall, two stack words: object, id). Returns a
/// pointer or null in `eax`.
lf_checker_rt::export!(thiscall, rw_00A71AB0(this: u32, obj: u32, id: u32) -> u32 {
    unsafe {
        const IDLES_OFF: u32 = 0x70;
        const JUMP_FLAG: u32 = 0x270;
        const JUMP_KEEP: u32 = 0xdfff_ffff;
        const MELEE_LINK: u32 = 0x398;
        const REC_FLAGS: u32 = 0x28;
        const FLAG_MASK: u32 = 0x3c0;
        const FLAG_WANT: u32 = 0xc0;
        const ID_JUMP: u32 = 0xd3;
        const ID_IDLES: u32 = 8;
        const ID_GUN: u32 = 6;
        const ID_CLIMB: u32 = 0x11f;
        const ID_MELEE: u32 = 0x1af;
        const ID_SEQ: u32 = 0x146;
        const POOL_GLOBAL: u32 = 0x167e2a0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn pool() -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(POOL_GLOBAL)) }
        }

        if (id as i32) > ID_CLIMB as i32 {
            if id == ID_SEQ {
                let a = lf_checker_rt::callee_thiscall!(15, u32, pool());
                if a == 0 {
                    return 0;
                }
                let b = lf_checker_rt::callee_thiscall!(16, u32, pool());
                let inner = if b == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(17, u32, b)
                };
                let c = lf_checker_rt::callee_thiscall!(18, u32, pool());
                let help = if c == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(19, u32, c)
                };
                return lf_checker_rt::callee_thiscall!(6, u32, a, help, inner, 1, 0);
            }
            if id == ID_MELEE {
                let t = rd32(obj + MELEE_LINK);
                let mut pick = 0u32;
                if t != 0 && rd32(t + REC_FLAGS) & FLAG_MASK == FLAG_WANT {
                    pick = t;
                }
                let a = lf_checker_rt::callee_thiscall!(13, u32, pool());
                if a == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(14, u32, a, 0, 0, 0, 0, 1, pick, 1);
            }
            return 0;
        }
        if id == ID_CLIMB {
            let a = lf_checker_rt::callee_thiscall!(11, u32, pool());
            if a == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(12, u32, a, 0);
        }
        if id == ID_GUN {
            let a = lf_checker_rt::callee_thiscall!(9, u32, pool());
            if a == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(10, u32, a);
        }
        if id == ID_IDLES {
            wr32(this + IDLES_OFF, 0);
            let a = lf_checker_rt::callee_thiscall!(7, u32, pool());
            if a == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(8, u32, a);
        }
        if id == ID_JUMP {
            wr32(obj + JUMP_FLAG, rd32(obj + JUMP_FLAG) & JUMP_KEEP);
            let a = lf_checker_rt::callee_thiscall!(1, u32, pool());
            if a == 0 {
                return 0;
            }
            let b = lf_checker_rt::callee_thiscall!(2, u32, pool());
            let inner = if b == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(3, u32, b, 0, 0)
            };
            let c = lf_checker_rt::callee_thiscall!(4, u32, pool());
            let help = if c == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(5, u32, c)
            };
            return lf_checker_rt::callee_thiscall!(6, u32, a, help, inner, 1, 0);
        }
        0
    }
});
