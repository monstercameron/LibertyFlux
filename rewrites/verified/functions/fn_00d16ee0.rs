// original: 0x00d16ee0 CTaskSimplePlayUpperCombatAnim::vf17
/// Advances the upper-combat animation state. The start helper's AL value
/// chooses state one or three; the animation lookup may advance state one
/// to two; a missing current subtask advances state two to three. State three
/// runs the finalizer and returns AL=1. State comparisons are unsigned.

const ANIMATION_GROUP_SECONDS_BITS: u32 = 0x4080_0000;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

lf_checker_rt::export!(thiscall, rw_00d16ee0(task: u32, entity: u32) -> u32 {
    unsafe {
        let starting_state = read_u32(task + 0x14);
        loop {
            match read_u32(task + 0x14) {
                0 => {
                    let ready = lf_checker_rt::callee_cdecl!(
                        1,
                        u32,
                        entity,
                        task + 0x24,
                        task + 0x28
                    );
                    let next_state = if ready & 0xFF != 0 { 1 } else { 3 };
                    (task as *mut u32).add(5).write_unaligned(next_state);
                }
                1 => {
                    let ready = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        task + 0x18,
                        read_u32(task + 0x24)
                    );
                    if ready & 0xFF != 0 {
                        let _factory_result = lf_checker_rt::callee_thiscall!(
                            3,
                            u32,
                            task,
                            entity,
                            read_u32(task + 0x24),
                            read_u32(task + 0x28),
                            ANIMATION_GROUP_SECONDS_BITS,
                            1
                        );
                        (task as *mut u32).add(5).write_unaligned(2);
                    }
                }
                2 => {
                    if read_u32(task + 0x10) == 0 {
                        (task as *mut u32).add(5).write_unaligned(3);
                    }
                }
                3 => {
                    let _finished = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        task + 0x18,
                        u32::MAX
                    );
                    let _finalizer = lf_checker_rt::callee_thiscall!(4, u32, task);
                    return 1;
                }
                _ => return 0,
            }

            if read_u32(task + 0x14) == starting_state {
                return 0;
            }
        }
    }
});
