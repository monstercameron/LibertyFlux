// original: 0x009BA690 CCamScriptInstruction_SetLookTargetOffset::vf2
/// Store a four-word look-target offset into a camera, by camera kind.
///
/// Resolves the target camera (`this+0x08`), reads its kind through virtual
/// slot `+0x28`: kind 14 stores the words at `this+0x10..0x1C` to target
/// `+0x200..0x20C`, kind 25 to `+0x170..0x17C`. All copies are bit for bit
/// (two integer words, two float words). Other kinds / null: nothing.
lf_checker_rt::export!(thiscall, rw_009BA690(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const KIND_SLOT: u32 = 0x28;
        const OFF_A: u32 = 0x200;
        const OFF_B: u32 = 0x170;
        const LOOKUP: u32 = 1;
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
        let dst = match kind_of(tgt) {
            14 => tgt.wrapping_add(OFF_A),
            25 => tgt.wrapping_add(OFF_B),
            _ => return 0,
        };
        for i in 0..4u32 {
            let w = (this.wrapping_add(0x10 + i * 4) as *const u32).read_unaligned();
            (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(w);
        }
        0
    }
});
