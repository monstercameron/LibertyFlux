// original: 0x00664430 sn_validate_join_request (proposed)

/// Validate a join request against the session, gather the member's data,
/// and submit it for synchronisation.
///
/// `this` is the task, `a0`/`a1`/`a2` are a tag, a flags word and a
/// request object. Two scratch quadwords are zeroed and two init callees run
/// against scratch buffers (their answers are ignored). Then a chain of
/// gates, each jumping to a shared failure exit: the task words at `+0x90`
/// and `+0x94` must be 3 and 1; the match callee (thiscall on scratch with
/// the request words at `+0x1c`/`+0x20` and the same scratch address, which
/// arrives as the previous stub's preserved register) must answer nonzero;
/// the two zeroed scratch words must equal the context words at
/// `+0x540`/`+0x544`. The shared failure exit returns eax untouched:
/// the second init answer for the first gate, the match answer for the
/// second, and the (zero) scratch word for the third.
///
/// Past the gates, the fetch callee (thiscall on the context with the
/// request word at `+0x18`) supplies an entry or null; a slot address is
/// formed from the task word at `+0x908` (`+0xa0` plus 64 times the word)
/// and the slot callee runs on scratch. A null entry, or a slot whose
/// words at `+0x38`/`+0x3c` differ from the entry's at `+0x40`/`+0x44`,
/// takes a short path that stores the two context words and a zero byte
/// and jumps to the submit step. Otherwise the log callee (thiscall on
/// the context with a scratch address, 0x70, 1 and a fixed image address)
/// and the count callee (thiscall on the context at `+0xbb0` with a
/// scratch address and the context) run; the count answer (SIGNED: zero
/// or negative copies nothing) selects that many pairs from scratch
/// (stride 0x70) into a second scratch area (stride 8). The use callee
/// (thiscall on the slot with `a2`), the pack callee (thiscall on the
/// task at `+0x910` with scratch) and an interlocked compare-exchange of
/// the `+0x94` word (3 for 1, clearing the word after on success) follow.
/// The submit callee (thiscall on `a0` with the request and the buffer)
/// runs on every non-failure path and its answer is the return.
///
/// The pair copy reads uninitialised scratch (zero under the checker's
/// zero stack fill); answers above 1 would drive its bound from
/// above-frame caller residue (the bound slot overlaps pair 1's
/// destination), which the checker cannot reproduce, so tested answers
/// stay at 0 or 1 (see narrowed).
///
/// Original: 0x00664430 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00664430(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x60;
        const LOG_CONST: u32 = 0x006B_D700;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        // Scratch mirror of the original's frame (zero stack fill).
        let mut s = [0u32; 64];
        let esi = a2;
        let ctx = rd32(this.wrapping_add(CTX));
        let cookie = || -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(12, u32,) }
        };

        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, core::ptr::addr_of!(s[20]) as u32);
        let ans2: u32 = lf_checker_rt::callee_thiscall!(2, u32, core::ptr::addr_of!(s[16]) as u32);
        // The shared failure exit returns eax untouched, so the first
        // gate returns the second init callee's answer.
        if rd32(this.wrapping_add(0x90)) != 3 || rd32(this.wrapping_add(0x94)) != 1 {
            cookie();
            return ans2;
        }
        let ebx = this.wrapping_add(0x94);
        // Note the this-pointer is the word below the preserved address
        // (the lea runs after two pushes).
        let ans3: u32 = lf_checker_rt::callee_thiscall!(
            3, u32, core::ptr::addr_of!(s[14]) as u32, rd32(esi.wrapping_add(0x1c)),
            rd32(esi.wrapping_add(0x20)), core::ptr::addr_of!(s[16]) as u32
        );
        if (ans3 as u8) == 0 {
            cookie();
            return ans3;
        }
        if s[14] != rd32(ctx.wrapping_add(0x540)) || s[15] != rd32(ctx.wrapping_add(0x544)) {
            cookie();
            return 0;
        }
        let ans4: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, ctx, rd32(esi.wrapping_add(0x18)));
        let k = rd32(this.wrapping_add(0x908));
        let slot = this.wrapping_add(0xa0).wrapping_add(k.wrapping_mul(64));
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, core::ptr::addr_of!(s[8]) as u32);
        if ans4 == 0
            || rd32(slot.wrapping_add(0x38)) != rd32(ans4.wrapping_add(0x40))
            || rd32(slot.wrapping_add(0x3c)) != rd32(ans4.wrapping_add(0x44))
        {
            s[8] = rd32(ctx.wrapping_add(0x540));
            s[9] = rd32(ctx.wrapping_add(0x544));
            // A single zero byte; the other three bytes keep their fill.
            s[13] &= 0xFFFF_FF00;
        } else {
            // The stores interleave with the log callee's pushes, so
            // they land one word lower than a plain reading suggests.
            s[8] = rd32(ctx.wrapping_add(0x540));
            s[9] = rd32(ctx.wrapping_add(0x544));
            let _: u32 = lf_checker_rt::callee_thiscall!(
                6, u32, ctx, core::ptr::addr_of!(s[34]) as u32, 0x70, 1,
                lf_checker_rt::relocated(LOG_CONST)
            );
            // A single flag byte set after the log call's pushes.
            s[13] = (s[13] & 0xFFFF_FF00) | 1;
            // The address lea runs after one push, four bytes lower.
            let ans7: u32 = lf_checker_rt::callee_thiscall!(
                7, u32, ctx.wrapping_add(0xbb0), core::ptr::addr_of!(s[34]) as u32, ctx
            );
            s[12] = ans7;
            if (ans7 as i32) > 0 {
                let mut i = 0u32;
                // The bound is re-read every iteration, like the original
                // (with tested answers it never changes mid-loop).
                while (i as i32) < (s[12] as i32) {
                    let src = 50usize + (i as usize) * 28;
                    let dst = 10usize + (i as usize) * 2;
                    let t0 = s[src];
                    let t1 = s[src + 1];
                    s[dst] = t0;
                    s[dst + 1] = t1;
                    i += 1;
                }
            }
            // The this-pointer is reloaded after the push, so it is the
            // saved first argument, not the slot.
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, a0, a1);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                9, u32, this.wrapping_add(0x910), core::ptr::addr_of!(s[16]) as u32
            );
            let ans10: u32 = lf_checker_rt::callee_stdcall!(10, u32, ebx, 3, 1);
            if ans10 == 1 {
                wr32(ebx.wrapping_add(4), 0);
            }
        }
        let ans11: u32 = lf_checker_rt::callee_thiscall!(
            11, u32, a0, esi, core::ptr::addr_of!(s[8]) as u32
        );
        cookie();
        ans11
    }
});
