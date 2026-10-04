// original: 0x00da8eb0 CTaskComplexStuckInAir::vf19
/// Forward a state event for the ped to the task's event handler.
///
/// Reads the ped through the task and dispatches event code 0xD0 when the
/// ped's state word says 2, otherwise event code 0xCB, always with the task
/// itself as the second argument. Returns the handler's answer.
export!(stdcall, rw_00da8eb0(task: u32) -> u32 {
    unsafe {
        /// Offset of the ped pointer inside the task.
        const PED: u32 = 0x224;
        /// Offset of the state word inside the ped.
        const STATE: u32 = 0x244;
        /// State value selecting the first event code.
        const AIRBORNE: u16 = 2;
        /// Event codes forwarded to the handler.
        const EV_AIR: u32 = 0xD0;
        const EV_GROUND: u32 = 0xCB;
        let p = ((task.wrapping_add(PED)) as *const u32).read_unaligned();
        let ev = if ((p.wrapping_add(STATE)) as *const u16).read_unaligned() == AIRBORNE {
            EV_AIR
        } else {
            EV_GROUND
        };
        callee_stdcall!(1, u32, ev, task)
    }
});
