// original: 0x00da87c0 CTaskComplexSmartFleePoint::vf7
/// Initialise the shared flee helper from this task's data block.
///
/// Same shape as the sibling slot: fetch the helper through the global slot
/// and, when available, initialise it from the data at `this + 0x30`.
export!(thiscall, rw_00da87c0(this: u32, _arg: u32) -> u32 {
    unsafe {
        /// Global slot holding the shared helper's manager.
        const MANAGER: u32 = 0x0171FAF4;
        /// Offset of this task's data block passed to the initialiser.
        const DATA: u32 = 0x30;
        let mgr = callee_thiscall!(1, u32, global::<u32>(MANAGER).read());
        if mgr == 0 {
            0
        } else {
            callee_thiscall!(2, u32, mgr, this.wrapping_add(DATA), 1, 0, 0)
        }
    }
});
