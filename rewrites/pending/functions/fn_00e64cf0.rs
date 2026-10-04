// original: 0x00e64cf0 SLO_MO_BULLET_BY_IN_1_MT
/// Resolve 6 audio events to their runtime handles and cache them.
///
/// Each event name (`SLO_MO_BULLET_BY_IN_1_MT`, `SLO_MO_BULLET_BY_OUT_1_MT`, `SLO_MO_BULLET_BY_IN_2_MT`, `SLO_MO_BULLET_BY_OUT_2_MT`, `SLO_MO_BULLET_BY_IN_3_MT`, `SLO_MO_BULLET_BY_OUT_3_MT`) is looked up through the shared
/// name-lookup routine and its handle stored in the matching slot.
/// Returns the last handle.
export!(cdecl, rw_00e64cf0() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8E808), 0);
        *global::<u32>(0x01283208) = r0;
        let r1 = callee_cdecl!(1, u32, relocated(0x00E8E824), 0);
        *global::<u32>(0x0128320C) = r1;
        let r2 = callee_cdecl!(1, u32, relocated(0x00E8E840), 0);
        *global::<u32>(0x01283210) = r2;
        let r3 = callee_cdecl!(1, u32, relocated(0x00E8E85C), 0);
        *global::<u32>(0x01283214) = r3;
        let r4 = callee_cdecl!(1, u32, relocated(0x00E8E878), 0);
        *global::<u32>(0x01283218) = r4;
        let r5 = callee_cdecl!(1, u32, relocated(0x00E8E894), 0);
        *global::<u32>(0x0128321C) = r5;
        r5
    }
});
