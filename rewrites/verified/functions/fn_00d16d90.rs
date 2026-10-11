// original: 0x00d16d90 CTaskSimplePlayAnimAndSlideOutOfCover::vf17
/// Starts the animation when the task is not already active, updates the
/// entity's task flags while a subtask exists, and returns AL as the done
/// flag. The engine state is a global word; the 4.0 duration is passed as
/// its original IEEE-754 bits to the intercepted subtask factory.

const TASK_ACTIVE_FLAG: u32 = 1 << 2;
const ENTITY_TASK_FLAG: u32 = 0x2000;
const SUBTASK_SIGNAL_FLAG: u32 = 0x200;
const ENGINE_MODE_VA: u32 = 0x011D_6FD4;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    (address as *mut u32).write_unaligned(value);
}

lf_checker_rt::export!(thiscall, rw_00d16d90(task: u32, entity: u32) -> u32 {
    unsafe {
        ((task + 0x30) as *mut u8).write(0);
        let flags = read_u32(task + 0x0C);
        if flags & TASK_ACTIVE_FLAG == 0 {
            let duration = read_u32(task + 0x1C);
            let _factory_result = lf_checker_rt::callee_thiscall!(
                1,
                u32,
                task,
                entity,
                read_u32(task + 0x14),
                read_u32(task + 0x18),
                duration,
                1
            );

            let mode = lf_checker_rt::global::<u32>(ENGINE_MODE_VA).read_unaligned();
            let current_subtask = read_u32(task + 0x10);
            if mode == 2
                && ((entity + 0x218) as *const u8).read() == 0
                && ((entity + 0x219) as *const u8).read() != 0
                && current_subtask != 0
            {
                write_u32(current_subtask + 0x54, 0x3FA6_6666);
            }
        }

        let current_subtask = read_u32(task + 0x10);
        if current_subtask == 0 {
            return 1;
        }
        let entity_flags = read_u32(entity + 0x29C);
        write_u32(entity + 0x29C, entity_flags | ENTITY_TASK_FLAG);
        write_u32(entity + 0x284, read_u32(task + 0x50));
        let signal_object = read_u32(entity + 0xA80);
        let signal_flags = read_u32(signal_object + 0x50);
        write_u32(signal_object + 0x50, signal_flags | SUBTASK_SIGNAL_FLAG);
        0
    }
});
