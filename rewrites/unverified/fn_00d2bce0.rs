// original: 0x00D2BCE0 CTaskComplexMoveFollowNavMeshRoute::vf20 (symbols)
//
// STAGE 1 of a staged rewrite: covers the function entry, the flag-gated
// early tail through this task's own vtable slot 0x4c, and the direct-call
// return path. Everything past the third gate (the long decision cascade)
// panics; the stage-1 contract pins the gates so it is unreachable.

/// Route-follow task update, stage 1: entry gating and two early exits.
///
/// `this` is the task, `arg` the update context. When state bit 0x200 is
/// set the context word at `+0x29c` gains bit 0x800, and when bit 0x20 is
/// set too a zero context at `+0xb0`, or a non-positive one at `+0xb8`,
/// stores a constant word at `+0x290`. Then three gates must all pass (bit
/// 1 of the word at `+0xc`, state bit 0x200 again, and a non-null table at
/// `+0x70`); the stage-1 contract pins them so the cascade past them is
/// out of scope. A direct helper is asked for a count from the context:
/// a zero count runs a three-word probe (whose nonzero answer, like a
/// count at or above the table's own count, takes the early tail that
/// sets state bit 0x100000 and dispatches through this task's vtable
/// slot 0x4c), while a count below the table's count records its low
/// byte at `+0x98` and returns through three direct calls (a one-word
/// notifier, a four-word query whose answer feeds a two-word follow-up).
///
/// Original: 0x00D2BCE0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d2bce0(this: u32, arg: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0xD8;
        const COUNT_HELPER: u32 = 1;
        const PROBE: u32 = 2;
        const NOTIFY: u32 = 3;
        const QUERY: u32 = 4;
        const FOLLOW: u32 = 5;
        const TAIL_SLOT: u32 = 0x4C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            (a as *const u32).read_unaligned()
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            (a as *mut u32).write_unaligned(v)
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            (a as *mut u8).write(v)
        }
        #[inline(always)]
        unsafe fn rd8s(a: u32) -> i32 {
            (a as *const u8).read() as i8 as i32
        }

        if rd32(this + STATE) & 0x200 != 0 {
            wr32(arg + 0x29C, rd32(arg + 0x29C) | 0x800);
            if rd32(this + STATE) & 0x20 != 0 {
                if rd32(arg + 0xB0) == 0 || (rd32(arg + 0xB8) as i32) <= 0 {
                    wr32(arg + 0x290, 0x3EC0_0000);
                }
            }
        }
        // Stage-1 gates: the contract pins all three taken.
        if (rd32(this + 0xC) >> 1) & 1 == 0 {
            panic!("r-b403 FN4 stage-1 boundary (gate 1)");
        }
        if rd32(this + STATE) & 0x200 == 0 {
            panic!("r-b403 FN4 stage-1 boundary (gate 2)");
        }
        let tab = rd32(this + 0x70);
        if tab == 0 {
            panic!("r-b403 FN4 stage-1 boundary (gate 3)");
        }
        let c1: u32 = lf_checker_rt::callee_stdcall!(COUNT_HELPER, u32, arg);
        let c: u32;
        if c1 == 0 {
            let a: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, this, arg, 1u32, 0u32);
            if (a as u8) != 0 {
                wr32(this + STATE, rd32(this + STATE) | 0x100000);
                let vt = rd32(this);
                let tail: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute::<u32, extern "thiscall" fn(u32, u32) -> u32>(
                        rd32(vt + TAIL_SLOT),
                    );
                return tail(this, arg);
            }
            c = c1;
        } else {
            c = c1;
        }
        if (c as i32) < (rd32(tab) as i32) {
            wr8(this + 0x98, c as u8);
            let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, arg);
            let b0 = rd32(this + STATE) & 1;
            let idx = rd8s(this + 0x98);
            let e: u32 =
                lf_checker_rt::callee_thiscall!(QUERY, u32, this, arg, idx as u32, b0, tab);
            let r: u32 = lf_checker_rt::callee_thiscall!(FOLLOW, u32, this, e, arg);
            return r;
        }
        wr32(this + STATE, rd32(this + STATE) | 0x100000);
        let vt = rd32(this);
        let tail: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute::<u32, extern "thiscall" fn(u32, u32) -> u32>(rd32(
                vt + TAIL_SLOT,
            ));
        tail(this, arg)
    }
});
