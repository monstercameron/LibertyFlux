// original: 0x009BA950 CCamScriptInstruction_SetRoll::vf2
/// Store the roll operand into a kind-14 camera.
///
/// Resolves the target camera (`this+0x08`); when it exists, reads its kind
/// through virtual slot `+0x28`, and only for kind 14 copies the word at
/// `this+0x0C` to target `+0x250`. Other kinds / null: nothing.
lf_checker_rt::export!(thiscall, rw_009BA950(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const KIND_SLOT: u32 = 0x28;
        const ROLL: u32 = 0x250;
        const KIND_ROLL: u32 = 14;
        const LOOKUP: u32 = 1;
        let tid = (this.wrapping_add(TARGET_ID) as *const u32).read_unaligned();
        let tgt =
            lf_checker_rt::callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(CAM_MGR), tid);
        if tgt == 0 {
            return 0;
        }
        let vtable = (tgt as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        if kind_of(tgt) == KIND_ROLL {
            let v = (this.wrapping_add(OPERAND) as *const u32).read_unaligned();
            (tgt.wrapping_add(ROLL) as *mut u32).write_unaligned(v);
        }
        0
    }
});
