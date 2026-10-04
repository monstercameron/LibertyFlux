// original: 0x00da8780 CTaskComplexFleeShooting::vf7
/// Initialise the shared flee helper, or forward the argument when empty.
///
/// When this task holds a target pointer it fetches the shared helper and
/// initialises it from the target; with no target it forwards its own stack
/// argument to the fallback handler instead. Either way the callee's answer
/// is returned, or 0 when the helper is unavailable.
export!(thiscall, rw_00da8780(this: u32, arg: u32) -> u32 {
    unsafe {
        /// Global slot holding the shared helper's manager.
        const MANAGER: u32 = 0x0171FAF4;
        /// Offset of this task's target pointer.
        const TARGET: u32 = 0x14;
        let tgt = ((this.wrapping_add(TARGET)) as *const u32).read_unaligned();
        if tgt == 0 {
            callee_stdcall!(3, u32, arg)
        } else {
            let mgr = callee_thiscall!(1, u32, global::<u32>(MANAGER).read());
            if mgr == 0 {
                0
            } else {
                callee_thiscall!(2, u32, mgr, tgt, 1, 0, 0)
            }
        }
    }
});
