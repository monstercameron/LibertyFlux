// original: 0x009CF4A0 traced_state_reset (proposed)

/// Reset the shared state object through a fixed sequence of traced steps.
///
/// Fetches singleton A (callee 1); when it is null the function does nothing
/// and returns 0. Otherwise it walks a fixed list of fields on the state
/// object returned by callee 2 (a fresh fetch before every step): enable
/// bytes at `+0x30b`/`+0x30a` gate two callback slots at `+0x384`/`+0x388`
/// (each invoked with an argument built by callees 4/5 when its slot is
/// non-null, then its enable byte is cleared); bytes at `+0x30c` and
/// `+0x306` and the dword at `+0x4` are set or cleared; a virtual call through
/// slot `+8` of singleton A's table reports error `0x80040904` (callee 8)
/// when it returns non-zero; when the low byte of the argument is non-zero a
/// pointer at `+0x8` is handed to callee 9 and then cleared; 32 bytes at
/// `+0x0c` are zeroed; and callee 10 clears `0x130` bytes at `+0x2c`.
///
/// Every step is wrapped in a pair of trace hooks (callee 3 reached through
/// two address-table slots) that run only while the flag byte at `FLAG` is
/// non-zero; both hooks take the same constant address argument. The hooks
/// are loaded once and reused. The return value is whatever the last executed
/// call returned (0 on the early path).
///
/// Original: 0x009CF4A0 (cdecl, one stack word; only its low byte is read).
lf_checker_rt::export!(cdecl, rw_009CF4A0(arg0: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x0103AD58;
        const HOOK_PRE_SLOT: u32 = 0x00E731CC;
        const HOOK_POST_SLOT: u32 = 0x00E731C8;
        const HOOK_ARG: u32 = 0x0129588C;
        const EN_CB1: u32 = 0x30b;
        const PTR_CB1: u32 = 0x384;
        const EN_CB2: u32 = 0x30a;
        const PTR_CB2: u32 = 0x388;
        const MARK_B: u32 = 0x30c;
        const MARK_A: u32 = 0x306;
        const FIELD_4: u32 = 0x4;
        const FIELD_8: u32 = 0x8;
        const ZERO_AT: u32 = 0x0c;
        const ZERO_WORDS: u32 = 8;
        const MEMSET_AT: u32 = 0x2c;
        const MEMSET_LEN: u32 = 0x130;
        const VTABLE_SLOT: u32 = 0x8;
        const ERR_CODE: u32 = 0x80040904;
        const GET_A: u32 = 1;
        const GET_OBJ: u32 = 2;
        #[allow(dead_code)]
        const HOOK: u32 = 3;
        const MAKE_CB1: u32 = 4;
        const MAKE_CB2: u32 = 5;
        // Callee ids reached through planted slots rather than the stub table.
        #[allow(dead_code)]
        const CALLBACK: u32 = 6;
        #[allow(dead_code)]
        const VTABLE_FN: u32 = 7;
        const REPORT_ERROR: u32 = 8;
        const NOTIFY: u32 = 9;
        const MEMSET: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32h(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8h(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8h(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32h(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn flag() -> u8 {
            unsafe { (lf_checker_rt::relocated(FLAG) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn hook(ptr: u32, arg: u32) -> u32 {
            unsafe {
                let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(ptr as usize);
                f(arg)
            }
        }
        #[inline(always)]
        unsafe fn get_obj() -> u32 {
            lf_checker_rt::callee_cdecl!(GET_OBJ, u32,)
        }

        let inst = lf_checker_rt::callee_cdecl!(GET_A, u32,);
        if inst == 0 {
            return 0;
        }
        let hook_arg = lf_checker_rt::relocated(HOOK_ARG);
        // Step 1: mark present.
        let obj = get_obj();
        let hook_pre = rd32(HOOK_PRE_SLOT);
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        wr8h(obj + MARK_B, 1);
        let hook_post = rd32(HOOK_POST_SLOT);
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        // Step 2: first gated callback, then clear its enable.
        let obj = get_obj();
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        let en = rd8h(obj + EN_CB1);
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        if en == 1 {
            let obj = get_obj();
            if flag() != 0 {
                hook(hook_pre, hook_arg);
            }
            let cb = rd32h(obj + PTR_CB1);
            if flag() != 0 {
                hook(hook_post, hook_arg);
            }
            if cb != 0 {
                let o = get_obj();
                let a = lf_checker_rt::callee_thiscall!(MAKE_CB1, u32, o);
                let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(cb as usize);
                f(a);
            }
            let obj = get_obj();
            if flag() != 0 {
                hook(hook_pre, hook_arg);
            }
            wr8h(obj + EN_CB1, 0);
            if flag() != 0 {
                hook(hook_post, hook_arg);
            }
        }
        // Step 3: second gated callback, then clear its enable.
        let obj = get_obj();
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        let en = rd8h(obj + EN_CB2);
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        if en == 1 {
            let obj = get_obj();
            if flag() != 0 {
                hook(hook_pre, hook_arg);
            }
            let cb = rd32h(obj + PTR_CB2);
            if flag() != 0 {
                hook(hook_post, hook_arg);
            }
            if cb != 0 {
                let o = get_obj();
                let a = lf_checker_rt::callee_thiscall!(MAKE_CB2, u32, o);
                let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(cb as usize);
                f(a);
            }
            let obj = get_obj();
            if flag() != 0 {
                hook(hook_pre, hook_arg);
            }
            wr8h(obj + EN_CB2, 0);
            if flag() != 0 {
                hook(hook_post, hook_arg);
            }
        }
        // Step 4: clear the mark.
        let obj = get_obj();
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        wr8h(obj + MARK_B, 0);
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        // Virtual status call; report on failure.
        let vt = rd32h(inst);
        let f: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(rd32h(vt + VTABLE_SLOT) as usize);
        if f(inst) != 0 {
            lf_checker_rt::callee_cdecl!(REPORT_ERROR, u32, ERR_CODE);
        }
        // Step 5: clear field +0x4.
        let obj = get_obj();
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        wr32h(obj + FIELD_4, 0);
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        // Step 6 (gated on the argument's low byte): notify, then clear +0x8.
        if (arg0 as u8) != 0 {
            let obj = get_obj();
            if flag() != 0 {
                hook(hook_pre, hook_arg);
            }
            let p = rd32h(obj + FIELD_8);
            if flag() != 0 {
                hook(hook_post, hook_arg);
            }
            if p != 0 {
                lf_checker_rt::callee_cdecl!(NOTIFY, u32, p);
            }
            let obj = get_obj();
            if flag() != 0 {
                hook(hook_pre, hook_arg);
            }
            wr32h(obj + FIELD_8, 0);
            if flag() != 0 {
                hook(hook_post, hook_arg);
            }
        }
        // Step 7: zero eight words at +0x0c.
        let obj = get_obj();
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        let mut i = 0u32;
        while i < ZERO_WORDS {
            wr32h(obj + ZERO_AT + i * 4, 0);
            i += 1;
        }
        // Step 8: clear a block at +0x2c through callee 10.
        let obj = get_obj();
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        let buf = obj + MEMSET_AT;
        if flag() != 0 {
            hook(hook_post, hook_arg);
        }
        lf_checker_rt::callee_cdecl!(MEMSET, u32, buf, 0u32, MEMSET_LEN);
        // Step 9: clear the last mark; the final call's value is returned.
        let obj = get_obj();
        if flag() != 0 {
            hook(hook_pre, hook_arg);
        }
        wr8h(obj + MARK_A, 0);
        if flag() != 0 {
            hook(hook_post, hook_arg)
        } else {
            obj
        }
    }
});
