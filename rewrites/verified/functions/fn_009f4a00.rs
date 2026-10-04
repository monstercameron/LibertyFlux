// original: 0x009f4a00 net_stage_04
/// Network stage 4 handler: stamp and register first, then fetch and report.
///
/// Snapshots the shared tick into stage 4's slot and registers the stage
/// (discarding the registrar's answer), runs the preamble, fetches an object
/// and reports it with a zero level. Returns the reporter's answer.
export!(cdecl, rw_009f4a00() -> u32 {{
    unsafe {{
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B61E0;
        /// Stage index passed to the registrar.
        const INDEX: u32 = 4;
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        let _: u32 = callee_cdecl!(1, u32, INDEX);
        let _: u32 = callee_cdecl!(2, u32,);
        let target: u32 = callee_cdecl!(3, u32, 0);
        callee_thiscall!(4, u32, target, 0)
    }}
}});
