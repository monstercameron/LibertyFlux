// original: 0x00DCD5F0 reload_task_advance_stage
/// Saves the current reload stage, marks the stage as active, and stores
/// the requested next stage. The task fields are at offsets 0x50, 0x54,
/// and 0x58; the method is thiscall with one stack argument.
lf_checker_rt::export!(thiscall, rw_00dcd5f0(task: u32, next_stage: u32) -> () {
    const ACTIVE_FLAG_OFFSET: usize = 0x50;
    const STAGE_OFFSET: usize = 0x54;
    const PREVIOUS_STAGE_OFFSET: usize = 0x58;
    // SAFETY: the checker contract supplies a writable task object.
    unsafe {
        let bytes = task as *mut u8;
        let stage = bytes.add(STAGE_OFFSET) as *mut u32;
        bytes.add(PREVIOUS_STAGE_OFFSET).cast::<u32>().write(stage.read());
        bytes.add(ACTIVE_FLAG_OFFSET).write(1);
        stage.write(next_stage);
    }
});
