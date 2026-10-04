// original: 0x008d8bf0 file_sync_drain_8bf0
/// Wait for readiness, sync shared state, then drain the work queue.
///
/// Polls the ready check until it reports success, runs the per-object and
/// three fixed-object sync stages, flips the generation counter, releases
/// the handle, and unless the done flag is raised, drains the queue (poll
/// until empty, servicing between polls) and runs the final stage. Returns
/// the last service answer.
///
/// Two ECX values are deliberately not compared (ids 4 and 9): the original
/// relies on ECX surviving the preceding thiscall, which holds for real
/// callees but not for the checker's recorder stubs, so the original side
/// observes stub garbage there. The rewrite passes the faithful constant.
export!(thiscall, rw_008d8bf0(this: u32) -> u32 {
    unsafe {
        /// Poll argument forwarded with the ready slot.
        const POLL_ARG: u32 = 0xFA0;
        /// Fixed `this` of the sync stages (file VA).
        const SYNC_OBJ: u32 = 0x011D4EC8;
        /// Fixed `this` of the queue stages (file VA).
        const QUEUE_OBJ: u32 = 0x0118D7F0;
        /// Done flag byte (file VA).
        const FLAG_DONE: u32 = 0x017ED8D2;
        /// State flag byte, pinned clear by the contract (file VA).
        const FLAG_STATE: u32 = 0x017ED8C1;
        /// Ready slot offset (words).
        const READY: usize = 0xFB4 / 4;
        /// Handle slot offset (words).
        const HANDLE: usize = 0xFB8 / 4;
        /// Generation counter offset (words).
        const GEN: usize = 0xFC4 / 4;
        let slots = this as *const u32;
        while callee_cdecl!(1, u32, slots.add(READY).read(), POLL_ARG) as u8 == 0 {}
        let _: u32 = callee_cdecl!(2, u32,);
        let _: u32 = callee_thiscall!(3, u32, relocated(SYNC_OBJ));
        let _: u32 = callee_thiscall!(4, u32, relocated(SYNC_OBJ));
        let gen = slots.add(GEN);
        (gen as *mut u32).write(1u32.wrapping_sub(gen.read()));
        let _: u32 = callee_thiscall!(5, u32, relocated(SYNC_OBJ));
        let _: u32 = callee_cdecl!(6, u32, slots.add(HANDLE).read());
        if global::<u8>(FLAG_DONE).read() != 0 {
            return callee_cdecl!(8, u32, 0);
        }
        // A raised state flag spins servicing until some callee clears it;
        // stubbed callees never do, so the contract pins it clear.
        while global::<u8>(FLAG_STATE).read() != 0 {
            let _: u32 = callee_cdecl!(8, u32, 0);
        }
        let queue = relocated(QUEUE_OBJ);
        if callee_thiscall!(7, u32, queue) as u8 == 0 {
            return callee_cdecl!(8, u32, 0);
        }
        loop {
            if global::<u8>(FLAG_STATE).read() == 0
                && callee_thiscall!(7, u32, queue) as u8 == 0
            {
                break;
            }
            let _: u32 = callee_cdecl!(8, u32, 0);
        }
        let _: u32 = callee_thiscall!(9, u32, queue);
        callee_cdecl!(8, u32, 0)
    }
});
