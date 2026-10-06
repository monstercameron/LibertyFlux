// original: 0x00661360 rage::snLeaveGamersFromRlineTask::vf2

/// Task step for removing gamers from a session line: either report through
/// the member slot or gather the leavers and hand them to the session.
///
/// `this` is the task: dword at `+0xc` an initialised flag, dword at `+0x60`
/// the session context, dword at `+0x90` a done flag, dword at `+0x94` a
/// result word, dword at `+0x2a0` a SIGNED leaver count, member slots at
/// `+0xa0` (stride 16).
///
/// When the flag is clear it is set. Then the session state (dword at
/// `+0x50` of the context, SIGNED) decides: below 2 or above 3, call the
/// task's own virtual slot `+0x1c` with (0, 0) and return its answer.
/// At 2 or 3, gather: zero a 31-slot scratch array, then for each leaver
/// call the fetch callee (thiscall on the context at `+0x48` with the
/// member slot); when it answers non-negative (SIGNED `js` test: negative
/// skips) copy the 16-byte slot into the next scratch slot. When at least
/// one slot filled, call the submit callee (thiscall on the same object
/// with the scratch base, the filled count and the result word's address);
/// when it answers with a zero low byte, or when nothing filled, exchange
/// 3 into the result word (an interlocked exchange) and clear the word
/// after it. Finally set the done flag and return the last answer.
///
/// Original: 0x00661360 (thiscall, no stack words; two code chunks).
lf_checker_rt::export!(thiscall, rw_00661360(this: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x0c;
        const CTX: u32 = 0x60;
        const STATE: u32 = 0x50;
        const CTX_OPS: u32 = 0x48;
        const DONE: u32 = 0x90;
        const RESULT: u32 = 0x94;
        const COUNT: u32 = 0x2a0;
        const SLOTS: u32 = 0xa0;
        const STRIDE: u32 = 16;
        const SCRATCH_SLOTS: usize = 31;
        const FETCH_ID: u32 = 2;
        const SUBMIT_ID: u32 = 3;
        const EXCHANGE_ID: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd32(this.wrapping_add(FLAG)) == 0 {
            wr32(this.wrapping_add(FLAG), 1);
        }
        let state = rd32(rd32(this.wrapping_add(CTX)).wrapping_add(STATE)) as i32;
        if state < 2 || state > 3 {
            let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(this).wrapping_add(0x1c)) as usize);
            return f(this, 0, 0);
        }

        let mut scratch = [0u32; SCRATCH_SLOTS * 4];
        let n = rd32(this.wrapping_add(COUNT)) as i32;
        let mut filled = 0u32;
        if n > 0 {
            let ops = rd32(this.wrapping_add(CTX)).wrapping_add(CTX_OPS);
            let mut k = 0u32;
            for _ in 0..n {
                let slot = this.wrapping_add(SLOTS).wrapping_add(k);
                let r: u32 = lf_checker_rt::callee_thiscall!(FETCH_ID, u32, ops, slot);
                if (r as i32) >= 0 {
                    let base = (filled as usize) * 4;
                    for w in 0..4usize {
                        scratch[base + w] = rd32(slot.wrapping_add((w as u32) * 4));
                    }
                    filled += 1;
                }
                k = k.wrapping_add(STRIDE);
            }
        }
        let mut ans: u32;
        if filled == 0 {
            ans = lf_checker_rt::callee_stdcall!(
                EXCHANGE_ID,
                u32,
                this.wrapping_add(RESULT),
                3
            );
            wr32(this.wrapping_add(RESULT).wrapping_add(4), 0);
        } else {
            let ops = rd32(this.wrapping_add(CTX)).wrapping_add(CTX_OPS);
            let r: u32 = lf_checker_rt::callee_thiscall!(
                SUBMIT_ID,
                u32,
                ops,
                core::ptr::addr_of_mut!(scratch) as u32,
                filled,
                this.wrapping_add(RESULT)
            );
            ans = r;
            if (r as u8) == 0 {
                ans = lf_checker_rt::callee_stdcall!(
                    EXCHANGE_ID,
                    u32,
                    this.wrapping_add(RESULT),
                    3
                );
                wr32(this.wrapping_add(RESULT).wrapping_add(4), 0);
            }
        }
        wr32(this.wrapping_add(DONE), 1);
        // The original's stack-cookie check preserves all registers; the
        // checker answers it with a preserving stub.
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        ans
    }
});
