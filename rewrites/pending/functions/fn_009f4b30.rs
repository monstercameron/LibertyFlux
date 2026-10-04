// original: 0x009f4b30 net_stage_0b
/// Network stage 11 handler: forward its argument word, stamp the shared tick, register.
 ///
/// Forwards the stage's argument word from `0x012FA1B8` through the first
/// callee, snapshots the shared tick at `0x011735B4` into this stage's slot
/// at `0x012B61FC`, then registers stage `11` (0xb) with the second
/// callee and returns its answer.
export!(cdecl, rw_009f4b30() -> u32 {
    unsafe {
        /// Stage argument word forwarded to the first callee (file VA).
        const ARG: u32 = 0x012FA1B8;
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B61FC;
        /// Stage index passed to the second callee.
        const INDEX: u32 = 0xb;
        let arg: u32 = global::<u32>(ARG).read();
        let _: u32 = callee_cdecl!(1, u32, arg);
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        callee_cdecl!(2, u32, INDEX)
    }
});
