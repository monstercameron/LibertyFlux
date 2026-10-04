// original: 0x009f4c70 net_stage_15
/// Network stage 21 handler: forward its argument word, stamp the shared tick, register.
 ///
/// Forwards the stage's argument word from `0x012FA368` through the first
/// callee, snapshots the shared tick at `0x011735B4` into this stage's slot
/// at `0x012B6224`, then registers stage `21` (0x15) with the second
/// callee and returns its answer.
export!(cdecl, rw_009f4c70() -> u32 {
    unsafe {
        /// Stage argument word forwarded to the first callee (file VA).
        const ARG: u32 = 0x012FA368;
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B6224;
        /// Stage index passed to the second callee.
        const INDEX: u32 = 0x15;
        let arg: u32 = global::<u32>(ARG).read();
        let _: u32 = callee_cdecl!(1, u32, arg);
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        callee_cdecl!(2, u32, INDEX)
    }
});
