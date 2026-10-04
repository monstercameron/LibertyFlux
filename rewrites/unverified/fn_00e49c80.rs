// original: 0x00E49C80 E1_SELECT
/// Select-menu builder, STAGE 1 of a staged verification: the entry gate.
///
/// The object is asked through its vtable slot 0x140 whether the menu is
/// already built; a nonzero answer skips the whole body and the gate's
/// answer is the leftover return value. This stage's contract pins the gate
/// answer nonzero, so only this path executes (1200/1200 trials, one branch
/// shape). The fallthrough body (float setup plus seven panel blocks) is
/// stage 2+ and intentionally faults if ever reached, so a widened contract
/// cannot silently pass. See the lane report for the full block map and the
/// stage-2 retry kit.
export!(thiscall, rw_00E49C80(this: u32) -> u32 {
    let edi = this;
    let vtbl = unsafe { (edi as *const u32).read() };
    let target = unsafe { ((vtbl.wrapping_add(0x140)) as *const u32).read() };
    let gate: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    let a = gate(edi);
    if (a as u8) != 0 {
        a
    } else {
        panic!("r-b165 stage 1: menu body not staged yet")
    }
});
