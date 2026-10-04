// original: 0x009f4c30 net_stage_13
/// Network stage 19 handler: forward its argument word, stamp the shared tick, register.
 ///
/// Forwards the stage's argument word from `0x012FA3D4` through the first
/// callee, snapshots the shared tick at `0x011735B4` into this stage's slot
/// at `0x012B621C`, then registers stage `19` (0x13) with the second
/// callee and returns its answer.
export!(cdecl, rw_009f4c30() -> u32 {
    unsafe {
        /// Stage argument word forwarded to the first callee (file VA).
        const ARG: u32 = 0x012FA3D4;
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B621C;
        /// Stage index passed to the second callee.
        const INDEX: u32 = 0x13;
        let arg: u32 = global::<u32>(ARG).read();
        let _: u32 = callee_cdecl!(1, u32, arg);
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        callee_cdecl!(2, u32, INDEX)
    }
});
