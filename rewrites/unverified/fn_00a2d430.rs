// original: 0x00a2d430 player_ped_dispatch_subtask_primary

/// Dispatch two caller-supplied words through a target helper with ECX set
/// to the task subobject at `this+0x228+0x70`. The helper also receives
/// `this+0x20+0x30`, both words, and a final literal one, in that order.
/// The input method is thiscall with two callee-cleaned stack words.
lf_checker_rt::export!(thiscall, rw_a2d430(this: u32, first: u32, second: u32) -> u32 {
    let task = unsafe { ((this + 0x0228) as *const u32).read_unaligned() };
    let manager = if task == 0 { 0 } else { task.wrapping_add(0x70) };
    let base_owner = unsafe { ((this + 0x0020) as *const u32).read_unaligned() };
    let subtask_base = base_owner.wrapping_add(0x30);
    unsafe { lf_checker_rt::callee_thiscall!(1, u32, manager, subtask_base, first, second, 1u32) }
});
