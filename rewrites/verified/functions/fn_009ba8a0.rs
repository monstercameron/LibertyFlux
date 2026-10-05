// original: 0x009BA8A0 CCamScriptInstruction_SetPosTargetEntity::vf2
/// Forward the position-target entity to a camera's setter.
///
/// Resolves the camera by id (`this+0x08`); when it exists, calls `callee 2`
/// (thiscall/1) with the word at `this+0x0C`. Null lookup: nothing.
lf_checker_rt::export!(thiscall, rw_009BA8A0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const CAM_ID: u32 = 0x08;
        const ENTITY: u32 = 0x0C;
        const LOOKUP: u32 = 1;
        const SET_ENTITY: u32 = 2;
        let id = (this.wrapping_add(CAM_ID) as *const u32).read_unaligned();
        let cam = lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), id);
        if cam != 0 {
            let e = (this.wrapping_add(ENTITY) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(SET_ENTITY, u32, cam, e);
        }
        0
    }
});
