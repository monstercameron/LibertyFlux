// original: 0x009f4fe0 net_stage_1d
/// Network stage 0x1d handler: five-argument setup, info lookup chain, stamp, register.
///
/// Fetches an object and runs a five-argument setup on it (object + 0x2B0 as
/// the receiver), looks up an info record for id 0x29, passes it through two
/// more callees, then stamps the shared tick into stage 0x1d's slot and
/// registers. Returns the registrar's answer.
export!(cdecl, rw_009f4fe0() -> u32 {{
    unsafe {{
        /// Shared tick snapshotted into the slot (file VA).
        const TICK: u32 = 0x011735B4;
        /// Slot this stage stamps (file VA).
        const SLOT: u32 = 0x012B6244;
        /// Stage index passed to the registrar.
        const INDEX: u32 = 0x1D;
        /// Info id looked up (passed twice).
        const INFO_ID: u32 = 0x29;
        let obj: u32 = callee_cdecl!(1, u32, 0);
        let _: u32 = callee_thiscall!(2, u32, obj.wrapping_add(0x2B0),
                                      INFO_ID, 1, 0, 0, 0);
        let _stale: u32 = global::<u32>(0x012B4138).read();
        let info: u32 = callee_cdecl!(3, u32, INFO_ID);
        let used: u32 = callee_thiscall!(4, u32, info);
        let _: u32 = callee_cdecl!(5, u32, used);
        let tick: u32 = global::<u32>(TICK).read();
        global::<u32>(SLOT).write(tick);
        callee_cdecl!(6, u32, INDEX)
    }}
}});
