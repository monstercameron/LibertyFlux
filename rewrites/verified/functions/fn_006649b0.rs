// original: 0x006649B0 rage::snChangeAttributesTask::vf7

/// Task step for changing session attributes: when active, snapshot the
/// attribute block and submit it; always release the session, then report.
///
/// `this` is the task: dword at `+0x60` is the session context (zeroed by
/// the end), dword at `+0x49c` a state flag. `a0` selects the phase
/// (1 = active), `a1` is forwarded to the release callee.
///
/// When `a0` is 1 and the session state (dword at `+0x50` of the context,
/// SIGNED) is 2 or 3 and both attribute stamp pairs match (dwords at
/// `+0xbf0`/`+0xc30` and `+0xbf4`/`+0xc34`), prepare a scratch buffer:
/// call the init callee on it, store the two header words (context
/// `+0x540`/`+0x544`), copy 259 dwords from context `+0x128`, and call
/// the submit callee (thiscall on the context with the buffer and 0: the
/// count register still holds 259 for the copy, but the repeat prefix
/// drains it, so the callee always sees 0). Otherwise, when the state
/// flag is 1, call the detach
/// callee (thiscall on the context at `+0x48` with the flag's address).
/// Then call the release callee (thiscall on the task with (`a0`, `a1`))
/// and clear the context word. When `a0` is 1 and the saved context's
/// state is still 2 or 3, call the report callee (thiscall on the saved
/// context with a small out-struct holding a magic, a self-pointer and
/// two zero words) and return its answer; when the state is outside 2..3
/// return the state itself (reloaded into eax); otherwise return the
/// release callee's answer.
///
/// Original: 0x006649B0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_006649B0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x60;
        const STATE: u32 = 0x50;
        const FLAG: u32 = 0x49c;
        const CTX_OPS: u32 = 0x48;
        const WORDS: usize = 259;
        const INIT_ID: u32 = 1;
        const SUBMIT_ID: u32 = 2;
        const DETACH_ID: u32 = 3;
        const RELEASE_ID: u32 = 4;
        const REPORT_ID: u32 = 5;
        const COOKIE_ID: u32 = 6;
        const REPORT_MAGIC: u32 = 0x00FE_32A8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let mut ans: u32;
        if a0 == 1 {
            let ctx = rd32(this.wrapping_add(CTX));
            let st = rd32(ctx.wrapping_add(STATE)) as i32;
            // Note the asymmetry: a state outside 2..3 falls to the detach
            // check, but mismatched stamps skip it (straight to release).
            if st >= 2 && st <= 3 {
                if rd32(ctx.wrapping_add(0xbf0)) == rd32(ctx.wrapping_add(0xc30))
                    && rd32(ctx.wrapping_add(0xbf4)) == rd32(ctx.wrapping_add(0xc34))
                {
                    let mut buf = [0u32; 2 + WORDS];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    INIT_ID,
                    u32,
                    core::ptr::addr_of_mut!(buf) as u32
                );
                buf[0] = rd32(ctx.wrapping_add(0x540));
                buf[1] = rd32(ctx.wrapping_add(0x544));
                for k in 0..WORDS {
                    buf[2 + k] = rd32(ctx.wrapping_add(0x128).wrapping_add((k as u32) * 4));
                }
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        SUBMIT_ID,
                        u32,
                        ctx,
                        core::ptr::addr_of_mut!(buf) as u32,
                        0
                    );
                }
            } else if rd32(this.wrapping_add(FLAG)) == 1 {
                let ops = rd32(this.wrapping_add(CTX)).wrapping_add(CTX_OPS);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    DETACH_ID,
                    u32,
                    ops,
                    this.wrapping_add(FLAG)
                );
            }
        } else if rd32(this.wrapping_add(FLAG)) == 1 {
            let ops = rd32(this.wrapping_add(CTX)).wrapping_add(CTX_OPS);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(DETACH_ID, u32, ops, this.wrapping_add(FLAG));
        }

        let saved = rd32(this.wrapping_add(CTX));
        ans = lf_checker_rt::callee_thiscall!(RELEASE_ID, u32, this, a0, a1);
        wr32(this.wrapping_add(CTX), 0);
        if a0 == 1 {
            let stw = rd32(saved.wrapping_add(STATE));
            if (stw as i32) >= 2 && (stw as i32) <= 3 {
                let mut rep = [0u32; 4];
                // An image address (its immediate carries a relocation entry).
                rep[0] = lf_checker_rt::relocated(REPORT_MAGIC);
                rep[1] = core::ptr::addr_of_mut!(rep) as u32;
                ans = lf_checker_rt::callee_thiscall!(
                    REPORT_ID,
                    u32,
                    saved,
                    core::ptr::addr_of_mut!(rep) as u32
                );
            } else {
                // The state reload lands in eax, which the skips preserve.
                ans = stw;
            }
        }
        // The original's stack-cookie check preserves all registers; the
        // checker answers it with a preserving stub.
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_ID, u32,);
        ans
    }
});
