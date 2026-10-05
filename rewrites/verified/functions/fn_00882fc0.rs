// original: 0x00882fc0 stream_req_close (proposed)
/// Close a streaming request: flush direct state, release owned objects.
///
/// Returns 0 at once when the open flag at `this+0x04` is clear. Otherwise
/// clears that flag and, when the direct flag at `this+0x30` is set, flushes
/// the two direct words (`+0x24` through intercepted callee 1, `+0x28`
/// through intercepted callee 2, both cdecl, one argument), clears the
/// direct flag, waits for the busy flag at `this+0x32` to clear (a
/// cross-thread spin the checker cannot execute; the contract pins it clear
/// and exempts intercepted callee 3), then releases word `+0x24`
/// (intercepted callee 4, cdecl, one argument) and zeroes it. Then, when the
/// owned object at `[this]` is non-null, calls its slot-2 handler
/// (intercepted callee 5, thiscall, no stack arguments) and, still non-null,
/// its slot-0 destructor with argument 1 (intercepted callee 6, thiscall,
/// one argument), and zeroes `[this]`. Finally, when the word at `this+0x0c`
/// is non-null, settles it (intercepted callee 7, cdecl, one argument) and
/// zeroes it. Returns 1.
///
/// Return channel: the early path preserves the caller's upper `eax`, so the
/// contract compares only `al`.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00882fc0(this: u32) -> u32 {
    unsafe {
        const OPEN: u32 = 0x04;
        const DIRECT: u32 = 0x30;
        const DIRECT_A: u32 = 0x24;
        const DIRECT_B: u32 = 0x28;
        const OBJ_SLOT: u32 = 0x0c;
        const FLUSH_A_CALLEE: u32 = 1;
        const FLUSH_B_CALLEE: u32 = 2;
        const RELEASE_CALLEE: u32 = 4;
        const HANDLER_CALLEE: u32 = 5;
        const DTOR_CALLEE: u32 = 6;
        const SETTLE_CALLEE: u32 = 7;
        if ((this + OPEN) as *const u8).read() == 0 {
            return 0;
        }
        ((this + OPEN) as *mut u8).write(0);
        if ((this + DIRECT) as *const u8).read() != 0 {
            let a = ((this + DIRECT_A) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(FLUSH_A_CALLEE, u32, a);
            let b = ((this + DIRECT_B) as *const u32).read_unaligned();
            ((this + DIRECT) as *mut u8).write(0);
            let _: u32 = lf_checker_rt::callee_cdecl!(FLUSH_B_CALLEE, u32, b);
            // The original spins here until another thread clears +0x32;
            // single-threaded trials pin it clear (see contract).
            let word = ((this + DIRECT_A) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE_CALLEE, u32, word);
            ((this + DIRECT_A) as *mut u32).write_unaligned(0);
        }
        let owned = (this as *const u32).read_unaligned();
        if owned != 0 {
            let table = (owned as *const u32).read_unaligned();
            let handler = ((table + 8) as *const u32).read_unaligned();
            let run: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(handler as usize);
            run(owned);
            let owned = (this as *const u32).read_unaligned();
            if owned != 0 {
                let table = (owned as *const u32).read_unaligned();
                let dtor = (table as *const u32).read_unaligned();
                let destroy: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(dtor as usize);
                destroy(owned, 1);
            }
            (this as *mut u32).write_unaligned(0);
        }
        let slot = ((this + OBJ_SLOT) as *const u32).read_unaligned();
        if slot != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(SETTLE_CALLEE, u32, slot);
            ((this + OBJ_SLOT) as *mut u32).write_unaligned(0);
        }
        1
    }
});
