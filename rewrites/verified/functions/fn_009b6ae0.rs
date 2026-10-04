// original: 0x009B6AE0 resolve_and_forward_state (proposed)
/// Resolve the state object for `a0` and forward it through two calls.
///
/// Looks the object up, reads its state through the table slot with the
/// caller's slot address alongside, forwards both through the shared query,
/// re-resolves with the object and `a0`, notifies, then stores `a0` at `a1`
/// and returns `a1`. The original leaves the slot word on the stack for the
/// query to pop; the rewrite passes an equal-valued word and cleans normally,
/// with an identical net stack effect. thiscall.
lf_checker_rt::export!(thiscall, rw_009B6AE0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const STATE_SLOT: u32 = 0x28;
        let obj1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, a0);
        let table = (obj1 as *const u32).read_unaligned();
        let target = ((table + STATE_SLOT) as *const u32).read_unaligned();
        let mut slot = Box::new(a0);
        let f: extern "cdecl" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let st = f((&mut *slot as *mut u32) as u32);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, st, ((&*slot as *const u32) as u32));
        let r4: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, a0, obj1);
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, r4);
        (a1 as *mut u32).write_unaligned(a0);
        a1
    }
});
