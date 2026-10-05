// original: 0x009BA770 CCamScriptInstruction_SetLookTargetPos::vf2
/// Forward a position vector to a camera's look-target setter, by kind.
///
/// Resolves the target camera (`this+0x08`) with no null check, reads its
/// kind through virtual slot `+0x28`, and calls `callee 3` (kind 14) or
/// `callee 4` (kind 25), each thiscall/1 with a pointer to the four words at
/// `this+0x10`. Other kinds do nothing.
lf_checker_rt::export!(thiscall, rw_009BA770(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const POS: u32 = 0x10;
        const KIND_SLOT: u32 = 0x28;
        const LOOKUP: u32 = 1;
        const SET_A: u32 = 3;
        const SET_B: u32 = 4;
        let tid = (this.wrapping_add(TARGET_ID) as *const u32).read_unaligned();
        let tgt =
            lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), tid);
        let vtable = (tgt as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let pos = this.wrapping_add(POS);
        match kind_of(tgt) {
            14 => {
                lf_checker_rt::callee_thiscall!(SET_A, u32, tgt, pos);
            }
            25 => {
                lf_checker_rt::callee_thiscall!(SET_B, u32, tgt, pos);
            }
            _ => {}
        }
        0
    }
});
