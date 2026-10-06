// original: 0x006641E0 sn_sync_member_lists (proposed)

/// Reconcile two member lists and submit the survivors: gather candidate
/// pairs through the session, drop the ones already known, and hand the
/// rest to the sync callee.
///
/// `this` is a network task: dword at `+0x60` is the session context, dword
/// at `+0x1658` a SIGNED group count, groups at `+0x9b0` (stride 0x68, each
/// with a key at `+0`, a tag at `+4` and a skip flag bit 0 at `+0xc`).
/// The context holds a second list head (count at `+0x1e14`, pointers at
/// `+0x1d94`) and two header words at `+0x540`/`+0x544`.
///
/// For each group whose skip flag is clear, call the lookup callee
/// (thiscall on the context with the key and tag). When it answers
/// non-null with a positive (SIGNED) pair count at `+0x6c`, append that
/// many pairs (words at `+0x40`/`+0x44` of each entry pointed to from
/// `+0x68`) to a scratch list capped at 32 pairs (note the count word at
/// `+0x6c` is also list entry [1]: counts above 1 only survive when the
/// word itself is a readable pointer), then call the touch callee
/// (thiscall
/// on the context with the key, the tag, and the same key and tag again:
/// the caller reserves 16 bytes of argument space but pushes only 8, so
/// the last two words are the lookup call's stale arguments). Then gather
/// up to 32 pairs from the context's own
/// list the same way (its count compared UNSIGNED) and delete from it
/// every pair already in the scratch list (swap-with-last). When more
/// than one pair survives, build a header (the two context words, the
/// survivor count) followed by the survivors and call the sync callee
/// (thiscall on the context with the buffer and the context); otherwise
/// skip the call. Return the sync answer with its low byte set to 1
/// (1 when the call was skipped).
///
/// Original: 0x006641E0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_006641E0(this: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x60;
        const GROUPS: u32 = 0x1658;
        const GROUP_BASE: u32 = 0x9b0;
        const GROUP_STRIDE: u32 = 0x68;
        const CAP: u32 = 32;
        const LOOKUP_ID: u32 = 1;
        const TOUCH_ID: u32 = 2;
        const SYNC_ID: u32 = 3;
        const COOKIE_ID: u32 = 5;

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
            unsafe { (a as *const u8).read() }
        }

        // Scratch pair lists. The original prefills 62 of the 64 words
        // with -1 (rep movsd); the last two words keep the stack fill (0).
        // buf1 has 4 extra words: the survivor copy can reach 68 words
        // (the last two pairs land in scratch past the snapshot).
        let mut buf1 = [0xFFFF_FFFFu32; 68];
        buf1[62] = 0;
        buf1[63] = 0;
        buf1[64] = 0;
        buf1[65] = 0;
        buf1[66] = 0;
        buf1[67] = 0;
        let mut buf2 = [0xFFFF_FFFFu32; 64];
        buf2[62] = 0;
        buf2[63] = 0;

        let ctx = rd32(this.wrapping_add(CTX));
        // Stale eax carried to the `(an instruction of the original)` return when the sync call is
        // skipped: the touch callee's answer (0), the last gathered pair
        // word, or the last dedup search index. (The entry cookie value
        // never survives on tested trials: every trial runs a touch call
        // or a context gather. See the contract's aligned pin periods.)
        let mut stale = 0u32;
        let mut total = 0u32;
        let ngroups = rd32(this.wrapping_add(GROUPS)) as i32;
        if ngroups > 0 {
            for idx in 0..ngroups {
                let g = this
                    .wrapping_add(GROUP_BASE)
                    .wrapping_add((idx as u32).wrapping_mul(GROUP_STRIDE));
                if rd8(g.wrapping_add(0x0c)) & 1 != 0 {
                    continue;
                }
                let key = rd32(g);
                let tag = rd32(g.wrapping_add(4));
                let ans: u32 = lf_checker_rt::callee_thiscall!(LOOKUP_ID, u32, ctx, key, tag);
                let mut added = 0u32;
                if ans != 0 {
                    let n2 = rd32(ans.wrapping_add(0x6c)) as i32;
                    if n2 > 0 {
                        let lim = CAP.wrapping_sub(total);
                        let mut k = 0u32;
                        while k < lim && (k as i32) < n2 {
                            let e = rd32(ans.wrapping_add(0x68).wrapping_add(k.wrapping_mul(4)));
                            let dst = (total.wrapping_add(k) as usize) * 2;
                            buf1[dst] = rd32(e.wrapping_add(0x40));
                            buf1[dst + 1] = rd32(e.wrapping_add(0x44));
                            k += 1;
                        }
                        added = k;
                    }
                }
                total = total.wrapping_add(added);
                let t: u32 =
                    lf_checker_rt::callee_thiscall!(TOUCH_ID, u32, ctx, key, tag, key, tag);
                stale = t;
            }
        }

        let mut nleft = 0u32;
        let nctx = rd32(ctx.wrapping_add(0x1e14));
        if nctx != 0 {
            let mut k = 0u32;
            while k < CAP && k < nctx {
                let e = rd32(ctx.wrapping_add(0x1d94).wrapping_add(k.wrapping_mul(4)));
                buf2[(k as usize) * 2] = rd32(e.wrapping_add(0x40));
                buf2[(k as usize) * 2 + 1] = rd32(e.wrapping_add(0x44));
                stale = rd32(e.wrapping_add(0x44));
                k += 1;
            }
            nleft = k;
        } else {
            // The gather is skipped, so eax still holds the context loaded
            // just before the count test.
            stale = ctx;
        }
        // Delete survivors already known: for each scratch pair, find
        // the first equal context pair and swap the last one over it.
        // (Runs whenever the scratch list is non-empty, even with an
        // empty context list, where each pass just clears eax.)
        let mut i = 0u32;
        while (i as i32) < (total as i32) {
            stale = 0;
            if nleft > 0 {
                let want0 = buf1[(i as usize) * 2];
                let want1 = buf1[(i as usize) * 2 + 1];
                let mut a = 0u32;
                while a < nleft {
                    if buf2[(a as usize) * 2] == want0
                        && buf2[(a as usize) * 2 + 1] == want1
                    {
                        break;
                    }
                    a += 1;
                }
                stale = a;
                if a < nleft {
                    nleft -= 1;
                    buf2[(a as usize) * 2] = buf2[(nleft as usize) * 2];
                    buf2[(a as usize) * 2 + 1] = buf2[(nleft as usize) * 2 + 1];
                }
            }
            i += 1;
        }

        let ans: u32;
        if nleft > 1 {
            for w in 4..68usize {
                buf1[w] = 0xFFFF_FFFF;
            }
            buf1[0] = rd32(ctx.wrapping_add(0x540));
            buf1[1] = rd32(ctx.wrapping_add(0x544));
            buf1[2] = nleft;
            let mut s = 0u32;
            while s < nleft && s < CAP {
                buf1[4 + (s as usize) * 2] = buf2[(s as usize) * 2];
                buf1[4 + (s as usize) * 2 + 1] = buf2[(s as usize) * 2 + 1];
                s += 1;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                SYNC_ID,
                u32,
                ctx,
                core::ptr::addr_of_mut!(buf1) as u32,
                ctx
            );
            ans = (r & 0xFFFF_FF00) | 1;
        } else {
            ans = (stale & 0xFFFF_FF00) | 1;
        }
        // The original's stack-cookie check preserves all registers; the
        // checker answers it with a preserving stub.
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_ID, u32,);
        ans
    }
});
