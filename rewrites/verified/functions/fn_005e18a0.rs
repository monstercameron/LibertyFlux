// original: 0x005E18A0 proposed_ped_phase_global_notification_check
/// Run two unconditional preparation helpers, then inspect the relocated
/// shared phase pointer. If it is non-null, ask the phase object at offset
/// `0x2b0` whether notification is ready and return that call's result. A
/// null global pointer leaves the second preparation helper's eax as the
/// incidental return value. The checked contract scripts readiness false;
/// its later byte gate and notification call are outside this proof.
lf_checker_rt::export!(cdecl, rw_005e18a0() -> u32 {
    const GLOBAL_PHASE_FILE_VA: u32 = 0x018b_6ebc;
    const NOTIFICATION_CONTEXT_OFFSET: u32 = 0x2b0;

    let _ = lf_checker_rt::callee_cdecl!(1, u32,);
    let preparation_result = lf_checker_rt::callee_cdecl!(2, u32,);
    let global_phase = lf_checker_rt::global::<u32>(GLOBAL_PHASE_FILE_VA);
    let phase = unsafe { global_phase.read_unaligned() };
    if phase == 0 {
        return preparation_result;
    }
    let context = phase.wrapping_add(NOTIFICATION_CONTEXT_OFFSET);
    lf_checker_rt::callee_thiscall!(3, u32, context, 1u32)
});
