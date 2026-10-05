// original: 0x009BABA0 CCamScriptInstruction_SetScreenFade::vf2
/// Build a screen fade from ten operands, then attach it to a slot.
///
/// Calls `callee 1` (thiscall/10) with (word at `+0x08`, word at `+0x10`,
/// word at `+0x1C`, byte at `+0x14`, 0, pointer to `this+0x18`, float at
/// `+0x20`, float at `+0x24`, byte at `+0x28`, 0), all as raw words. Then
/// computes slot = answer + 4 * (9 * word_at_+0x0C + 0x109) with wrapping
/// arithmetic and calls `callee 2` (thiscall/0) with it. No branches.
lf_checker_rt::export!(thiscall, rw_009BABA0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const BUILD: u32 = 1;
        const ATTACH: u32 = 2;
        const SLOT_BASE: u32 = 0x109;
        let w08 = (this.wrapping_add(0x08) as *const u32).read_unaligned();
        let w10 = (this.wrapping_add(0x10) as *const u32).read_unaligned();
        let w1c = (this.wrapping_add(0x1C) as *const u32).read_unaligned();
        let b14 = (this.wrapping_add(0x14) as *const u8).read() as u32;
        let vec = this.wrapping_add(0x18);
        let f20 = (this.wrapping_add(0x20) as *const u32).read_unaligned();
        let f24 = (this.wrapping_add(0x24) as *const u32).read_unaligned();
        let b28 = (this.wrapping_add(0x28) as *const u8).read() as u32;
        let r = lf_checker_rt::callee_thiscall!(
            BUILD, u32, lf_checker_rt::relocated(CAM_MGR),
            w08, w10, w1c, b14, 0, vec, f20, f24, b28, 0);
        let idx = (this.wrapping_add(0x0C) as *const u32).read_unaligned();
        let slot = r.wrapping_add(idx.wrapping_mul(9).wrapping_add(SLOT_BASE).wrapping_mul(4));
        lf_checker_rt::callee_thiscall!(ATTACH, u32, slot);
        0
    }
});
