// original: 0x00BB85B0 CTaskSimpleCombatRoll::CTaskSimpleCombatRoll

/// Runs the simple-task base constructor, stores the supplied combat-roll
/// value at offset 0x14, installs the combat-roll vtable, and returns the
/// receiver. The base constructor takes no stack arguments; this constructor
/// pops its one 32-bit argument.
lf_checker_rt::export!(thiscall, rw_bb85b0(this: u32, roll_value: u32) -> u32 {
    unsafe {
        const BASE_TASK_CTOR: u32 = 1;
        const COMBAT_ROLL_VTABLE: u32 = 0x00EB7634;
        const VALUE_FIELD: u32 = 0x14;

        let _ = lf_checker_rt::callee_thiscall!(BASE_TASK_CTOR, u32, this);
        (this.wrapping_add(VALUE_FIELD) as *mut u32).write_unaligned(roll_value);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(COMBAT_ROLL_VTABLE));
        this
    }
});
