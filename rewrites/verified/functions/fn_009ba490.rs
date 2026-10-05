// original: 0x009BA490 CCamScriptInstruction_SetInterpGraphTypeRot::vf2
/// Set the rotation interpolation graph type on a camera.
///
/// Same shape as the position variant, but forwards channel 0 (rotation).
/// Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA490(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const GRAPH: u32 = 0x0C;
        const CHANNEL_ROT: u32 = 0;
        const LOOKUP: u32 = 1;
        const SET_GRAPH: u32 = 2;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let g = (this.wrapping_add(GRAPH) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(SET_GRAPH, u32, cam, CHANNEL_ROT, g);
        }
        0
    }
});
