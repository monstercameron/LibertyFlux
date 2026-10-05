// original: 0x00be74f0 CTaskSimpleSetCharCombatDecisionMaker::vf17

/// Forward the task's stored combat-decision-maker handle to the ped's brain.
///
/// `this` points to the task, `ped` to the ped. Loads the handle at `+0x14`,
/// resolves the ped's decision structure at `ped+0x224` and invokes the
/// engine setter (callee 1, thiscall, one stack word) with it. Always returns
/// 1. Twin of `CTaskSimpleSetCharDecisionMaker::vf17` with a different setter.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be74f0(this: u32, ped: u32) -> u32 {
    unsafe {
        const OFF_HANDLE: u32 = 0x14;
        const PED_BRAIN: u32 = 0x224;
        const SETTER: u32 = 1;

        let handle = ((this + OFF_HANDLE) as *const u32).read_unaligned();
        let brain = ((ped + PED_BRAIN) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(SETTER, u32, brain, handle);
        1
    }
});
