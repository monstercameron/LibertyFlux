// original: 0x009B9F10 CCamScriptInstruction_SetCamSplineCustomSpeedGraph::vf2

/// Execute the SetCamSplineCustomSpeedGraph script instruction: append the
/// operand at `this+0x08` to the engine's custom-speed-graph slot array.
///
/// The slot count global is incremented first and the operand is stored at
/// the new count's index (`slots[count]` after `count += 1`), so slot 0 is
/// never written. No calls, no return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009B9F10(this: u32) -> u32 {
    unsafe {
        const OPERAND: u32 = 0x08;
        const COUNT: u32 = 0x016D_8DB0;
        const SLOTS: u32 = 0x016D_8D5C;
        let next = lf_checker_rt::global::<u32>(COUNT).read().wrapping_add(1);
        let value = ((this + OPERAND) as *const u32).read_unaligned();
        lf_checker_rt::global::<u32>(SLOTS).add(next as usize).write(value);
        lf_checker_rt::global::<u32>(COUNT).write(next);
        0
    }
});
