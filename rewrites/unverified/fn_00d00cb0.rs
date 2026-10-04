// original: 0x00d00cb0 CTaskComplexCombat::vf8

/// Combat-task teardown step: notifies the child task, then runs two
/// follow-up calls on the given object.
///
/// `this` (ECX) is the combat task, `arg` the subject object. When the child
/// pointer at `this + 8` is non-null, the child's virtual slot at `+0x20` is
/// invoked with ECX = child and one stack word = `arg` (intercepted through
/// a fabricated object on both sides). Then the notifier (callee 2, thiscall,
/// no stack args) runs with ECX = `arg`, and the reset (callee 3, thiscall,
/// no stack args) with ECX = the dword at `arg + 0x224`. No return value.
///
/// Original: 0x00d00cb0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d00cb0(this: u32, arg: u32) -> u32 {
    unsafe {
        const CHILD_SLOT: u32 = 8;
        const CHILD_NOTIFY_VT: u32 = 0x20;
        const INNER_SLOT: u32 = 0x224;
        const CHILD_CALLEE: u32 = 1;
        const NOTIFY_CALLEE: u32 = 2;
        const RESET_CALLEE: u32 = 3;
        let _ = CHILD_CALLEE;
        let child = (this.wrapping_add(CHILD_SLOT) as *const u32).read_unaligned();
        if child != 0 {
            let vt = (child as *const u32).read_unaligned();
            let target = (vt.wrapping_add(CHILD_NOTIFY_VT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            f(child, arg);
        }
        lf_checker_rt::callee_thiscall!(NOTIFY_CALLEE, u32, arg);
        let inner = (arg.wrapping_add(INNER_SLOT) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(RESET_CALLEE, u32, inner);
        0
    }
});
