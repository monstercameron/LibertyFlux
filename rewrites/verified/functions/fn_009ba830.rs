// original: 0x009BA830 CCamScriptInstruction_SetPos::vf2
/// Set a camera's position, by camera kind, with a direct fallback.
///
/// Resolves the target camera (`this+0x08`); when it exists, reads its kind
/// through virtual slot `+0x28`. Kind 1 forwards a pointer to the four words
/// at `this+0x10` to `callee 3` and returns; otherwise the kind is read again
/// and kind 2 forwards the same pointer to `callee 4` and returns; otherwise
/// the four words are copied bit for bit to target `+0x40..0x4C`.
/// Note the original queries the kind twice (the second query reuses the
/// object pointer in ecx); both queries hit the same slot.
lf_checker_rt::export!(thiscall, rw_009BA830(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const POS: u32 = 0x10;
        const KIND_SLOT: u32 = 0x28;
        const POS_DST: u32 = 0x40;
        const LOOKUP: u32 = 1;
        const SET_A: u32 = 3;
        const SET_B: u32 = 4;
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
        let pos = this.wrapping_add(POS);
        if kind_of(tgt) == 1 {
            lf_checker_rt::callee_thiscall!(SET_A, u32, tgt, pos);
            return 0;
        }
        if kind_of(tgt) == 2 {
            lf_checker_rt::callee_thiscall!(SET_B, u32, tgt, pos);
            return 0;
        }
        for i in 0..4u32 {
            let w = (pos.wrapping_add(i * 4) as *const u32).read_unaligned();
            (tgt.wrapping_add(POS_DST + i * 4) as *mut u32).write_unaligned(w);
        }
        0
    }
});
