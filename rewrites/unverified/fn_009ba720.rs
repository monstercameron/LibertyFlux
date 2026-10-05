// original: 0x009BA720 CCamScriptInstruction_SetLookTargetOffsetRelative::vf2
/// Set the offset-relative flag on a camera, by camera kind.
///
/// Resolves the target camera (`this+0x08`), reads its kind through virtual
/// slot `+0x28`. Kind 14 writes bit 0 of the byte at `this+0x0C` into bit 1
/// of the flag byte at target `+0x264` (other bits preserved); kind 25
/// copies the whole byte to target `+0x180`. Other kinds / null: nothing.
lf_checker_rt::export!(thiscall, rw_009BA720(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const OPERAND: u32 = 0x0C;
        const KIND_SLOT: u32 = 0x28;
        const FLAGS_A: u32 = 0x264;
        const RELATIVE_BIT: u8 = 0x02;
        const SLOT_B: u32 = 0x180;
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
        let b = (this.wrapping_add(OPERAND) as *const u8).read();
        match kind_of(tgt) {
            14 => {
                let fs = tgt.wrapping_add(FLAGS_A) as *mut u8;
                let old = fs.read();
                fs.write(if b & 1 != 0 { old | RELATIVE_BIT } else { old & !RELATIVE_BIT });
            }
            25 => {
                (tgt.wrapping_add(SLOT_B) as *mut u8).write(b);
            }
            _ => {}
        }
        0
    }
});
