// original: 0x00d16870 CTaskSimpleCombatRoll::vf17
//! Checks the task's active flag, optionally creates the selected combat-roll
//! subtask, and returns AL as a completion flag. The state comparison is
//! unsigned; states above three use the invalid action code. The intercepted
//! factory receives the entity, task kind, action, duration bits, and a flag.

const ACTIVE_FLAG: u32 = 1 << 2;
const FIRST_ACTION: u32 = 0xAB;
const COMBAT_ROLL_KIND: u32 = 0x8B;
const FOUR_SECONDS_BITS: u32 = 0x4080_0000;

lf_checker_rt::export!(thiscall, rw_00d16870(this: u32, entity: u32) -> u32 {
    let task_flags = unsafe { ((this + 0x0C) as *const u32).read_unaligned() };
    if task_flags & ACTIVE_FLAG != 0 {
        let current_subtask = unsafe { ((this + 0x10) as *const u32).read_unaligned() };
        return u32::from(current_subtask == 0);
    }

    let state = unsafe { ((this + 0x14) as *const u32).read_unaligned() };
    let action = if state <= 3 {
        FIRST_ACTION + state
    } else {
        u32::MAX
    };

    let _factory_result = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        entity,
        COMBAT_ROLL_KIND,
        action,
        FOUR_SECONDS_BITS,
        1
    );
    0
});
