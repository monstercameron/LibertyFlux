// original: 0x009f4a30 net_stage_06
/// Network stage 6 handler: two preambles, then stamp and register.
///
/// Runs two preamble callees, snapshots the shared tick into stage 6's slot,
/// registers the stage and returns the registrar's answer.
export!(cdecl, rw_009f4a30() -> u32 {{
    unsafe {{
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B61E8;
        /// Stage index passed to the registrar.
        const INDEX: u32 = 6;
        let _: u32 = callee_cdecl!(1, u32,);
        let _: u32 = callee_cdecl!(2, u32,);
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        callee_cdecl!(3, u32, INDEX)
    }}
}});
