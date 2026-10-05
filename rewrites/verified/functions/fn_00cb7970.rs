// original: 0x00cb7970 CTaskComplexMoveGoToShelterAndWait::vf6
/// Whether a go-to-shelter-and-wait task is in an active step (vf6).
///
/// Reads the step field at `+0x18` of the task object (`this`, thiscall).
/// Steps 1 and 2 count as active and return the field with its low byte
/// forced to 1 (the original's `(an instruction of the original)` keeps the upper bytes of the
/// field); any other step returns the field with its low byte cleared
/// (the original's `(an instruction of the original)` clears only the low byte). No calls.
lf_checker_rt::export!(thiscall, rw_00cb7970(this: u32) -> u32 {
    unsafe {
        /// Step field: 1 and 2 mean the task is still running.
        const STEP_OFF: u32 = 0x18;
        let step = ((this + STEP_OFF) as *const u32).read_unaligned();
        if step == 1 || step == 2 {
            (step & 0xFFFFFF00) | 1
        } else {
            step & 0xFFFFFF00
        }
    }
});
