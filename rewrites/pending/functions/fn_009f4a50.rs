// original: 0x009f4a50 net_stage_07
/// Network stage 7 handler with two early exits: a kill-switch byte and a state check.
 ///
/// Returns immediately when the kill-switch byte at `0x012BD0FD` is nonzero
/// (leaving the incoming EAX in place, which the contract pins to zero) or
/// when the fetched object's state word selects the skip path; otherwise runs
/// the same forward-stamp-register sequence as the plain stage handlers and
/// returns the registrar's answer.
export!(cdecl, rw_009f4a50() -> u32 {
    unsafe {
        /// Kill-switch byte: nonzero means return immediately (file VA).
        const FLAG: u32 = 0x012BD0FD;
        /// Stage argument word forwarded to the second callee (file VA).
        const ARG: u32 = 0x012FA56C;
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B61EC;
        /// Stage index passed to the registrar.
        const INDEX: u32 = 0x7;
        if global::<u8>(FLAG).read() != 0 {
            return 0;
        }
        let obj: u32 = callee_cdecl!(1, u32,);
        if ((obj + 0x26C) as *const u8).read() & 4 != 0 {
            let inner: u32 = ((obj + 0xB30) as *const u32).read();
            if inner != 0 && ((inner + 0x1304) as *const u32).read() == 3 {
                return inner;
            }
        }
        let arg: u32 = global::<u32>(ARG).read();
        let _: u32 = callee_cdecl!(2, u32, arg);
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        callee_cdecl!(3, u32, INDEX)
    }
});
