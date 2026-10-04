// original: 0x00d40690 CTaskComplexJump::vf5

/// Decide whether a complex jump task accepts the requested transition.
///
/// `this` is the task, `kind` (arg1) the requested transition, `extra`
/// (arg2) forwarded. The subtask at `SUB` (+8) whose flag byte at +0xc
/// has bit 0 set skips the subtask query. Otherwise the subtask's slot
/// +0x14 is called (thiscall on the subtask, three stack words): the
/// first two words the original pushes are a blend of its return address
/// with arg0 and a below-stack word, which no rewrite can observe, so the
/// contract compares only the third word (`extra`) and the rewrite passes
/// zeros; a zero low byte of the answer rejects, else bit 1 of the flag
/// byte is set. Accepting (either way) runs the settle callee (thiscall
/// shape with `this` and arg0) and returns 1; rejecting returns 0. Only
/// al carries the result. Arg1 is unread.
///
/// Original: 0x00d40690 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d40690(this: u32, a0: u32, _kind: u32, extra: u32) -> u32 {
    unsafe {
        const SUB: u32 = 8;
        const QUERY_SLOT: u32 = 0x14;
        const SETTLE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let task = this;
        let sub = rd32(task + SUB);
        if ((sub + 0x0c) as *const u8).read() & 1 == 0 {
            let vtable = rd32(sub);
            let slot = rd32(vtable + QUERY_SLOT);
            let query: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            if query(sub, 0, 0, extra) & 0xff == 0 {
                return 0;
            }
            let flags = (sub + 0x0c) as *mut u32;
            flags.write_unaligned(flags.read_unaligned() | 2);
        }
        lf_checker_rt::callee_thiscall!(SETTLE, u32, task, a0);
        1
    }
});
