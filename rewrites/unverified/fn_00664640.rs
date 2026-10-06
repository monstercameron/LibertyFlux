// original: 0x00664640 sn_task_member_update (proposed)

/// Run one member-update step of a session task.
///
/// `this` is the task, `a0` is unused, `a1` is the member record and `a2`
/// the request. The request word at `+0x24` dispatches: anything outside
/// 1..4 returns it unchanged; 1 gathers new member entries, 2 refreshes
/// one entry, 3 and 4 run the fetch and take the failure exit. For 1..4
/// the task counter at `+0x165c` is decremented first and the fetch callee
/// (thiscall on the context at `+0x60` with the member words at `+0x58`
/// and `+0x5c`) supplies the entry pointer `edi`; the member flag byte at
/// `+0x64` has bit 0 cleared before the fetch.
///
/// Path 1 (gather): a scratch match struct is zeroed with three `0xFFFFFFFF`
/// words and a zero flag byte, then the match callee (thiscall on scratch
/// with the request words at `+0x1c`/`+0x20`) runs; a zero answer, a zero
/// flag byte or a null entry takes the failure exit. Otherwise the flag
/// bit is set and two format callees and a parse callee fill a scratch
/// area; the parse answer (SIGNED: `jle`, zero or negative gathers
/// nothing) bounds an outer loop over stride-0x70 entries, each
/// deduplicated against the scratch pair list (bound re-read once, SIGNED
/// `jle`) and copied (16 bytes) when new. A nonzero kept count submits
/// the header plus entries through the submit callee (thiscall on the
/// context with the context word at `+0xbb0`, the buffer, the count and
/// 0/-1/0/0); its answer is the return.
///
/// Path 2 (refresh): a null entry or a negative entry word at `+0x0`
/// (SIGNED `jl`) fails; the member limit word at `+0x60` is incremented
/// and compared UNSIGNED (`jae`) against the image limit 6, failing when
/// reached. Otherwise three callees run (init on scratch, fill on
/// scratch+0xc with the context words at `+0x540`/`+0x544` and the context
/// address at `+0xd0`, update on the context at `+0xc6c` with the entry
/// word, a scratch address and the member); a zero update answer skips
/// the counter increment. The tail: a nonzero counter returns the live
/// value, else the notify callee (stdcall with `this+0x94`, 3, 1) runs
/// and an answer of 1 clears the word at `+0x98`.
///
/// The failure exit returns eax untouched: the mode for 3 and 4, the
/// match answer for a zero match, the entry (or zero) for a bad entry,
/// the incremented count for the limit gate, and the second format
/// answer for a non-positive bound. An empty gather reaches the tail
/// with the last inner scan index in eax.
/// Three pushed arguments are the previous stub's exit ecx (post-call
/// residue the rewrite cannot read); they are skipped in the contract and
/// the stubs ignore them. The two format immediates are relocated image
/// code addresses, resolved through the runtime.
///
/// Original: 0x00664640 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00664640(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x60;
        const COUNT: u32 = 0x165c;
        const LIMIT: u32 = 6;
        const FMT1: u32 = 0x006B_D700;
        const FMT2: u32 = 0x006B_D0E0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write_unaligned(v) }
        }

        // Scratch mirror of the original's frame under zero stack fill,
        // indexed as (address - frame base) / 4.
        let mut s = [0u32; 64];
        let _ = a0;
        let esi = a1;
        let ebx_req = a2;
        let cookie = || -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(11, u32,) }
        };

        let mode = rd32(ebx_req.wrapping_add(0x24));
        if mode < 1 || mode > 4 {
            cookie();
            return mode;
        }
        wr32(this.wrapping_add(COUNT), rd32(this.wrapping_add(COUNT)).wrapping_sub(1));
        let ctx = rd32(this.wrapping_add(CTX));
        let w5c = rd32(esi.wrapping_add(0x5c));
        wr8(esi.wrapping_add(0x64), rd8(esi.wrapping_add(0x64)) & 0xfe);
        let w58 = rd32(esi.wrapping_add(0x58));
        let edi: u32 =
            lf_checker_rt::callee_thiscall!(1, u32, ctx, w58, w5c);

        // Failure exit shared by both paths; the value is the live eax.
        macro_rules! fail {
            ($v:expr) => {{
                cookie();
                return $v;
            }};
        }
        // Refresh path plus shared tail; always returns. Reached for
        // mode 2 and for mode 1 with a zero flag byte.
        macro_rules! refresh {
            ($entry:expr) => {{
                let entry: u32 = $entry;
                if entry == 0 {
                    fail!(0);
                }
                let ew = rd32(entry);
                if (ew as i32) < 0 {
                    fail!(entry);
                }
                let n = rd32(esi.wrapping_add(0x60)).wrapping_add(1);
                wr32(esi.wrapping_add(0x60), n);
                if n >= LIMIT {
                    fail!(n);
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    7, u32, core::ptr::addr_of!(s[16]) as u32
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    8, u32, core::ptr::addr_of!(s[19]) as u32,
                    rd32(ctx.wrapping_add(0x540)), rd32(ctx.wrapping_add(0x544)),
                    ctx.wrapping_add(0xd0)
                );
                let ans9: u32 = lf_checker_rt::callee_thiscall!(
                    9, u32, ctx.wrapping_add(0xc6c), rd32(entry),
                    core::ptr::addr_of!(s[18]) as u32, 0, esi
                );
                let mut eax = ans9;
                if (ans9 as u8) != 0 {
                    wr32(this.wrapping_add(COUNT), rd32(this.wrapping_add(COUNT)).wrapping_add(1));
                }
                if rd32(this.wrapping_add(COUNT)) != 0 {
                    cookie();
                    return eax;
                }
                let ans10: u32 = lf_checker_rt::callee_stdcall!(
                    10, u32, this.wrapping_add(0x94), 3, 1
                );
                eax = ans10;
                if ans10 == 1 {
                    wr32(this.wrapping_add(0x98), 0);
                }
                cookie();
                return eax;
            }};
        }

        if mode == 2 {
            refresh!(edi);
        }
        if mode != 1 {
            // Modes 3 and 4: eax was reloaded with the mode for dispatch.
            fail!(mode);
        }

        // Gather path.
        s[10] = 0;
        s[11] = 0;
        s[12] = 0xFFFF_FFFF;
        s[13] = 0xFFFF_FFFF;
        s[14] = 0xFFFF_FFFF;
        s[15] = 0x0000_0000;
        let ans2: u32 = lf_checker_rt::callee_thiscall!(
            2, u32, core::ptr::addr_of!(s[10]) as u32,
            rd32(ebx_req.wrapping_add(0x1c)), rd32(ebx_req.wrapping_add(0x20)), 0
        );
        if (ans2 as u8) == 0 {
            fail!(ans2);
        }
        if (s[15] & 0xff) == 0 {
            // Merged refresh path (never taken in tests: the stub
            // always sets the flag byte).
            refresh!(edi);
        }
        if edi == 0 {
            fail!(0);
        }
        wr8(esi.wrapping_add(0x64), rd8(esi.wrapping_add(0x64)) | 1);
        let _: u32 = lf_checker_rt::callee_stdcall!(
            3, u32, core::ptr::addr_of!(s[16]) as u32, 0x70, 1,
            lf_checker_rt::relocated(FMT1)
        );
        let bound: u32 = lf_checker_rt::callee_thiscall!(
            4, u32, edi, core::ptr::addr_of!(s[16]) as u32, 0
        );
        s[3] = this;
        s[4] = bound;
        let ans5: u32 = lf_checker_rt::callee_stdcall!(
            5, u32, core::ptr::addr_of!(s[6]) as u32, 0x10, 1,
            lf_checker_rt::relocated(FMT2)
        );
        let mut kept = 0u32;
        s[5] = 0;
        if (bound as i32) <= 0 {
            // eax still holds the second format callee's answer.
            fail!(ans5);
        }
        let edx = s[14];
        let mut dst_w = 6usize;
        let mut src_w = 34usize;
        let mut rem = bound;
        // The inner scan index survives in eax when nothing is kept.
        let mut last_inner = 0u32;
        loop {
            let plo = s[src_w - 2];
            let phi = s[src_w - 1];
            let mut found = false;
            if (edx as i32) > 0 {
                let mut k = 0u32;
                loop {
                    if s[12 + (k as usize) * 2] == plo && s[13 + (k as usize) * 2] == phi {
                        found = true;
                        last_inner = k;
                        break;
                    }
                    k += 1;
                    if (k as i32) >= (edx as i32) {
                        last_inner = k;
                        break;
                    }
                }
            } else {
                last_inner = 0;
            }
            if !found {
                s[dst_w] = s[src_w];
                s[dst_w + 1] = s[src_w + 1];
                s[dst_w + 2] = s[src_w + 2];
                s[dst_w + 3] = s[src_w + 3];
                kept += 1;
                s[5] = kept;
                dst_w += 4;
            }
            src_w += 28;
            rem = rem.wrapping_sub(1);
            s[4] = rem;
            if rem == 0 {
                break;
            }
        }
        let mut eax: u32;
        if kept == 0 {
            eax = last_inner;
        } else {
            eax = lf_checker_rt::callee_thiscall!(
                6, u32, ctx, rd32(ctx.wrapping_add(0xbb0)),
                core::ptr::addr_of!(s[3]) as u32, kept, 0, 0, 0xFFFF_FFFF, 0
            );
        }
        if rd32(this.wrapping_add(COUNT)) != 0 {
            cookie();
            return eax;
        }
        let ans10: u32 =
            lf_checker_rt::callee_stdcall!(10, u32, this.wrapping_add(0x94), 3, 1);
        eax = ans10;
        if ans10 == 1 {
            wr32(this.wrapping_add(0x98), 0);
        }
        cookie();
        eax
    }
});
