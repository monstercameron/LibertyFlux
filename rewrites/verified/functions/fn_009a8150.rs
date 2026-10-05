// original: 0x009a8150 script_slot_clear
/// Clear script slot `idx`, logging the teardown when it is live.
///
/// A negative index returns at once. Otherwise the resolver (stubbed,
/// cdecl/0) supplies a table whose word at `+4` selects the slot's
/// owner, and the opener (stubbed, thiscall/1) is tried: a
/// non-negative answer clears the slot flag at `this + idx*16 + 0x33dc`
/// and returns. A negative answer runs the resetter (stubbed,
/// thiscall/1 with 0) and retries the opener; a non-negative retry
/// clears the same flag. When both attempts fail, the event source
/// (stubbed, cdecl/0) is polled: a null event returns, otherwise the
/// event is formatted into a stack buffer (stubbed, cdecl/3) and
/// logged with the slot index (stubbed, cdecl/5). The original guards
/// its stack with the CRT security cookie and checks it on exit
/// (stubbed, preserving all registers); the rewrite issues the same
/// check call for log parity but keeps no cookie of its own. Thiscall,
/// one stack word, no compared result (EAX is cookie-derived garbage
/// on the early path).
export!(thiscall, rw_009A8150(this: u32, idx: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x33dc;
        const STRIDE: u32 = 16;
        const LOG_TAG: u32 = 0xe9182c;
        if (idx as i32) < 0 {
            let _: u32 = callee_cdecl!(8, u32,);
            return 0;
        }
        let p: u32 = callee_cdecl!(1, u32,);
        let owner = ((p + 4) as *const u32).read_unaligned();
        let r1: u32 = callee_thiscall!(2, u32, this, owner);
        if (r1 as i32) >= 0 {
            ((this + idx * STRIDE + SLOTS) as *mut u8).write(0);
            let _: u32 = callee_cdecl!(8, u32,);
            return 0;
        }
        let _: u32 = callee_thiscall!(3, u32, this, 0);
        let r3: u32 = callee_thiscall!(4, u32, this, owner);
        if (r3 as i32) >= 0 {
            ((this + idx * STRIDE + SLOTS) as *mut u8).write(0);
            let _: u32 = callee_cdecl!(8, u32,);
            return 0;
        }
        let ev: u32 = callee_cdecl!(5, u32,);
        // The format buffer's content is never observed (both callees
        // are stubs and nothing snapshots it), so a plain local stands
        // in for the original's frame buffer; only its address shape
        // (a stack pointer, skipped in the log) is reproduced.
        let mut buf = [0u32; 32];
        let bufp = (&mut buf as *mut u32) as u32;
        let _: u32 = callee_cdecl!(6, u32, bufp, 0, 0x78);
        if ev != 0 {
            let _: u32 = callee_cdecl!(7, u32, bufp, 0x80, relocated(LOG_TAG), ev, r3);
        }
        let _: u32 = callee_cdecl!(8, u32,);
        0
    }
});
