// original: 0x00DF8720 gallery_record_sync (proposed)

/// Validate a record buffer with a retry loop (stdcall, one stack arg,
/// byte result).
///
/// Builds a scratch buffer through a vtable writer callee (which fills it
/// with a NUL-terminated string), checks it through two helper callees and
/// an unsigned length bound of 32, then binds it to a fresh object. Any
/// failure routes to a confirm callee whose nonzero byte answer retries
/// the whole pass; a bind failure releases the object first. Success
/// clears a flag and skips the confirm call. The tail releases through a
/// second vtable slot and a shared-context lookup; the result is the flag
/// masked by the final call succeeding (1 only when a confirm path ran
/// and the final call returned 0).
///
/// The writer's buffer is the function's own frame, so that argument is
/// skipped and its words snapshotted instead (four snapshots: pre-write,
/// post-write twice, and at confirm time). The confirm callee answers a
/// fixed retry sequence (retry, retry, done) so every trial exercises the
/// loop and terminates. The CRT security-cookie check runs natively.
lf_checker_lf_checker_rt::export!(stdcall, rb586_fn5(arg0: u32) -> u32 {
    unsafe {
        const K_SETUP: u32 = 0x00f01c3c;
        const K_NAME: u32 = 0x01168dd8;
        const K_SECOND: u32 = 0x00f01c44;
        const K_DONE: u32 = 0x00f01c4c;
        const CTX: u32 = 0x01981a4c;
        const S_WRITE: u32 = 0x58;
        const S_CONFIRM: u32 = 0x5c;
        const S_RELEASE: u32 = 0x60;
        const CTX_SLOT: u32 = 0x1f8;
        const LEN_BOUND: u32 = 0x20;
        const C_SETUP: u32 = 1;
        const C_MAKE: u32 = 2;
        const C_WRITE: u32 = 3;
        const C_PROBE: u32 = 4;
        const C_CHECK: u32 = 5;
        const C_NEW: u32 = 6;
        const C_BIND: u32 = 7;
        const C_TRY: u32 = 8;
        const C_DROP: u32 = 9;
        const C_CONFIRM: u32 = 10;
        const C_RELEASE: u32 = 11;
        const C_LOOKUP: u32 = 12;
        const C_FINAL: u32 = 13;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0)
            }
        }
        #[inline(always)]
        unsafe fn vcall2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0, a1)
            }
        }
        unsafe fn strlen(mut s: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while rd8(s) != 0 {
                    s = s.wrapping_add(1);
                    n = n.wrapping_add(1);
                }
                n
            }
        }

        lf_checker_rt::callee_cdecl!(C_SETUP, u32, lf_checker_rt::relocated(K_SETUP), 0);
        let edi: u32 = lf_checker_rt::callee_cdecl!(C_MAKE, u32, lf_checker_rt::relocated(K_NAME), 1);
        let mut flag = 1u32;
        let mut buf = [0u32; 10];
        let bufp = buf.as_mut_ptr() as u32;
        // The writer runs once; the retry loop below re-enters after it.
        let ebx: u32 = vcall2(edi, S_WRITE, lf_checker_rt::relocated(K_NAME), bufp);
        loop {
            let p: u32 = lf_checker_rt::callee_cdecl!(C_PROBE, u32, bufp);
            let ok: u32 = lf_checker_rt::callee_cdecl!(C_CHECK, u32, p, lf_checker_rt::relocated(K_SECOND));
            let mut failed = ok == 0;
            if !failed {
                if strlen(bufp) >= LEN_BOUND {
                    failed = true;
                } else {
                    let esi: u32 = lf_checker_rt::callee_cdecl!(C_NEW, u32,);
                    lf_checker_rt::callee_thiscall!(C_BIND, u32, esi, bufp);
                    let t: u32 = lf_checker_rt::callee_thiscall!(C_TRY, u32, esi, arg0);
                    if t != 0 {
                        flag = 0;
                        break;
                    }
                    lf_checker_rt::callee_cdecl!(C_DROP, u32, esi);
                    failed = true;
                }
            }
            if failed {
                let r: u32 = vcall2(edi, S_CONFIRM, ebx, bufp);
                if (r as u8) == 0 {
                    break;
                }
            }
        }
        vcall1(edi, S_RELEASE, ebx);
        let o: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, lf_checker_rt::relocated(CTX), lf_checker_rt::relocated(K_DONE));
        let last: u32 = lf_checker_rt::callee_thiscall!(C_FINAL, u32, rd32(o + CTX_SLOT), arg0);
        if last != 0 {
            0
        } else {
            flag
        }
    }
});
