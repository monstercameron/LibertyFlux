// original: 0x0065AF60 shader_bind_single_pass (proposed)

/// Copy one pass descriptor to scratch, bind it, and append the result.
///
/// Copies the argument descriptor into a 35-word scratch buffer (patched
/// callee, thiscall: buffer, argument; scripted in the proof), seeds the
/// flag byte at buffer `+0x6e` from the copied byte at `+0x58` and the word
/// at `+0x70` with 0. When the signed index at `arg + 0x44` is not below
/// zero (signed bound: -1 and below skip), lies below the unsigned-16-bit
/// count at `this + 0x18` (non-negative on both sides, so the signedness of
/// this second bound is unobservable), and the indexed pass pointer is
/// non-null, the flag becomes 1 and the word becomes that pointer. Then
/// runs the binder (slot `+0x38` of the global binder object, thiscall:
/// buffer `+4`, the words at `+0x48`..`+0x54`, buffer `+0x5c`; the two
/// buffer addresses are skipped in the proof and their contents snapshotted)
/// and returns null when it answers null. Otherwise appends through the
/// appender (thiscall on `this + 0x14`: the word surviving the binder call,
/// skipped as harness scratch), stores the bound pass in the appended slot,
/// runs the pass hook (slot `+0x14` of the answer), and pairs the answer
/// with the hook result (callee: the two scratch addresses, skipped and
/// snapshotted). The stack-cookie check runs on every path (preserving
/// callee) and the answer is returned (thiscall, one argument).
lf_checker_rt::export!(thiscall, rw_0065af60(this: u32, arg: u32) -> u32 {
    unsafe {
        const CALLEE_COPY: u32 = 1;
        const CALLEE_BIND: u32 = 2;
        const CALLEE_APPEND: u32 = 3;
        const CALLEE_PAIR: u32 = 5;
        const CALLEE_COOKIE: u32 = 6;
        const BINDER_OBJ: u32 = 0x17F5630;
        unsafe fn rd(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        let mut buf = [0u32; 35];
        let bp = buf.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_COPY, u32, bp, arg);
        let al = ((bp + 0x58) as *const u8).read_unaligned();
        ((bp + 0x6e) as *mut u8).write_unaligned(al);
        ((bp + 0x70) as *mut u32).write_unaligned(0);
        let idx = ((arg + 0x44) as *const i32).read_unaligned();
        if idx > -1 {
            let count = ((this + 0x18) as *const u16).read_unaligned() as i32;
            if idx < count {
                let arr = ((this + 0x14) as *const u32).read_unaligned();
                let pass = ((arr + (idx as u32) * 4) as *const u32).read_unaligned();
                if pass != 0 {
                    ((bp + 0x6e) as *mut u8).write_unaligned(1);
                    ((bp + 0x70) as *mut u32).write_unaligned(pass);
                }
            }
        }
        let gobj = *lf_checker_rt::global::<u32>(BINDER_OBJ);
        let gvt = (gobj as *const u32).read_unaligned();
        let gtgt = ((gvt + 0x38) as *const u32).read_unaligned();
        let bind: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(gtgt as usize);
        let ans = bind(
            gobj,
            bp.wrapping_add(4),
            rd(bp, 0x48),
            rd(bp, 0x4c),
            rd(bp, 0x50),
            rd(bp, 0x54),
            bp.wrapping_add(0x5c),
        );
        let ret = if ans == 0 {
            0
        } else {
            let slot: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_APPEND, u32, this.wrapping_add(0x14), 0);
            (slot as *mut u32).write_unaligned(ans);
            let pvt = (ans as *const u32).read_unaligned();
            let htgt = ((pvt + 0x14) as *const u32).read_unaligned();
            let hook: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(htgt as usize);
            let hookans = hook(ans);
            let a = ans;
            let b = hookans;
            let _: u32 = lf_checker_rt::callee_stdcall!(
                CALLEE_PAIR, u32, &a as *const u32 as u32, &b as *const u32 as u32);
            ans
        };
        let _: u32 = lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
        ret
    }
});
