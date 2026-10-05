// original: 0x009BA530 CCamScriptInstruction_SetInterpStyleDetailed::vf2
/// Apply a detailed interpolation style: two state words plus two graph types.
///
/// Resolves the target camera (`this+0x08`); when it exists, stores the words
/// at `this+0x14`/`+0x18` to target `+0x15C`/`+0x160`, then calls the graph
/// setter (`callee 2`, thiscall/2) twice: (1, word at `+0x0C`) for position
/// and (0, word at `+0x10`) for rotation. Null target: nothing.
lf_checker_rt::export!(thiscall, rw_009BA530(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const POS_GRAPH: u32 = 0x0C;
        const ROT_GRAPH: u32 = 0x10;
        const STATE0: u32 = 0x14;
        const STATE1: u32 = 0x18;
        const DST_STATE0: u32 = 0x15C;
        const DST_STATE1: u32 = 0x160;
        const LOOKUP: u32 = 1;
        const SET_GRAPH: u32 = 2;
        let mgr = lf_checker_rt::relocated(CAM_MGR);
        let tid = (this.wrapping_add(TARGET_ID) as *const u32).read_unaligned();
        let tgt = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, tid);
        if tgt != 0 {
            let s0 = (this.wrapping_add(STATE0) as *const u32).read_unaligned();
            let s1 = (this.wrapping_add(STATE1) as *const u32).read_unaligned();
            (tgt.wrapping_add(DST_STATE0) as *mut u32).write_unaligned(s0);
            (tgt.wrapping_add(DST_STATE1) as *mut u32).write_unaligned(s1);
            let pg = (this.wrapping_add(POS_GRAPH) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(SET_GRAPH, u32, tgt, 1, pg);
            let rg = (this.wrapping_add(ROT_GRAPH) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(SET_GRAPH, u32, tgt, 0, rg);
        }
        0
    }
});
