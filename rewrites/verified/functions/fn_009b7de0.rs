// original: 0x009B7DE0 CCamScriptInstruction_SequenceWait::vf1
/// Compares the signed elapsed tick count (current tick minus start tick, with 32-bit wrapping) against the signed wait duration stored in the CCamScriptInstruction_SequenceWait layout. Returns the ordered comparison in AL.
lf_checker_rt::export!(thiscall, rw_009b7de0(this: u32) -> u32 {
    const GAME_TICK_VA: u32 = 0x011735C4;
    const WAIT_DURATION: u32 = 0x0C;
    const START_TICK: u32 = 0x10;
    unsafe {
        let now = (lf_checker_rt::global::<u32>(GAME_TICK_VA)).read_unaligned();
        let start = (this.wrapping_add(START_TICK) as *const u32).read_unaligned();
        let duration = (this.wrapping_add(WAIT_DURATION) as *const u32).read_unaligned();
        ((now.wrapping_sub(start) as i32) >= duration as i32) as u32
    }
});
