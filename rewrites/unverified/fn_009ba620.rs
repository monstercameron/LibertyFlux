// original: 0x009BA620 CCamScriptInstruction_SetLookTargetEntity::vf2
/// Point a camera's look target at an entity, by camera kind.
///
/// Resolves the target camera (`this+0x08`), reads its kind through virtual
/// slot `+0x28`, and dispatches on it: kind 14 -> `callee 3`, kind 25 ->
/// `callee 4`, kind 1 -> `callee 5` (each thiscall/1 with the entity word at
/// `this+0x0C`), kind 2 stores the entity word directly at target `+0x170`.
/// Any other kind, or a null target, does nothing.
lf_checker_rt::export!(thiscall, rw_009BA620(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const ENTITY: u32 = 0x0C;
        const KIND_SLOT: u32 = 0x28;
        const ENTITY_SLOT: u32 = 0x170;
        const LOOKUP: u32 = 1;
        const SET_A: u32 = 3;
        const SET_B: u32 = 4;
        const SET_C: u32 = 5;
        let tid = (this.wrapping_add(TARGET_ID) as *const u32).read_unaligned();
        let tgt = lf_checker_rt::callee_thiscall!(
            LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), tid);
        if tgt == 0 {
            return 0;
        }
        let vtable = (tgt as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let ent = (this.wrapping_add(ENTITY) as *const u32).read_unaligned();
        match kind_of(tgt) {
            14 => {
                lf_checker_rt::callee_thiscall!(SET_A, u32, tgt, ent);
            }
            25 => {
                lf_checker_rt::callee_thiscall!(SET_B, u32, tgt, ent);
            }
            1 => {
                lf_checker_rt::callee_thiscall!(SET_C, u32, tgt, ent);
            }
            2 => {
                (tgt.wrapping_add(ENTITY_SLOT) as *mut u32).write_unaligned(ent);
            }
            _ => {}
        }
        0
    }
});
