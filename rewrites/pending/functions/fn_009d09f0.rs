// original: 0x009d09f0 hook_event_dispatcher
/// Dispatch a hook event: fast path returns 1, otherwise chain CallNextHookEx.
///
/// When the first arg is 0, the second is 0x100 or 0x101, the probe call
/// reports 0 and the pointed-to word is 0x5b or 0x5c, returns 1; otherwise
/// chains to the next hook with the combined result or the locked field.
export!(stdcall, rw_009d09f0(a0: u32, a1: u32, a2: u32) -> u32 {
    const LOCK_FLAG: u32 = 0x0103AD58;
    const LOCK_CS: u32 = 0x0129588C;
    let flag = global::<u8>(LOCK_FLAG);
    if a0 == 0 {
        if a1.wrapping_sub(0x100) <= 1 {
            let p: u32 = callee_cdecl!(3, u32,);
            let t: u32 = callee_thiscall!(4, u32, p);
            if (t & 0xff) == 0 {
                let w = unsafe { (a2 as *const u32).read() };
                if w == 0x5b || w == 0x5c {
                    return 1;
                }
            }
        }
        let q: u32 = callee_cdecl!(3, u32,);
        let r: u32 = callee_thiscall!(5, u32, q);
        return callee_stdcall!(6, u32, r, 0, a1, a2);
    }
    let p: u32 = callee_cdecl!(3, u32,);
    if unsafe { flag.read() } != 0 {
        callee_stdcall!(1, u32, relocated(LOCK_CS));
    }
    let guarded = unsafe { flag.read() };
    let v = unsafe { (p.wrapping_add(0x2bc) as *const u32).read() };
    if guarded != 0 {
        callee_stdcall!(2, u32, relocated(LOCK_CS));
    }
    callee_stdcall!(6, u32, v, a0, a1, a2)
});
