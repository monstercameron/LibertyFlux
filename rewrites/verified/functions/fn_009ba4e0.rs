// original: 0x009BA4E0 CCamScriptInstruction_SetInterpStyleCore::vf2
/// Apply a core interpolation style built from three camera lookups.
///
/// Resolves the target camera by id (`this+0x08`); when it exists, resolves
/// two style cameras by the ids at `this+0x10` and `this+0x0C`, then calls
/// `callee 2` (thiscall/5) on the target with (style_b, style_a, word at
/// `+0x14`, word at `+0x18`, 1). The later lookup answers are only forwarded
/// as data, never null-checked. Null target: nothing.
lf_checker_rt::export!(thiscall, rw_009BA4E0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const STYLE_B_ID: u32 = 0x0C;
        const STYLE_A_ID: u32 = 0x10;
        const PARAM0: u32 = 0x14;
        const PARAM1: u32 = 0x18;
        const CORE_FLAG: u32 = 1;
        const LOOKUP: u32 = 1;
        const SET_STYLE: u32 = 2;
        let mgr = lf_checker_rt::relocated(CAM_MGR);
        let tid = (this.wrapping_add(TARGET_ID) as *const u32).read_unaligned();
        let tgt = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, tid);
        if tgt != 0 {
            let aid = (this.wrapping_add(STYLE_A_ID) as *const u32).read_unaligned();
            let a = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, aid);
            let bid = (this.wrapping_add(STYLE_B_ID) as *const u32).read_unaligned();
            let b = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, bid);
            let p0 = (this.wrapping_add(PARAM0) as *const u32).read_unaligned();
            let p1 = (this.wrapping_add(PARAM1) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(SET_STYLE, u32, tgt, b, a, p0, p1, CORE_FLAG);
        }
        0
    }
});
