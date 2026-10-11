// original: 0x00DCD020 reload_task_initialize
/// Initializes a reload task after its base-task constructor returns. It
/// stores two float inputs bit-for-bit, fills four range fields with all-one
/// words, clears transient fields, and derives a two-step counter from bits
/// 27 and 28 of the identifier argument.
lf_checker_rt::export!(thiscall, rw_00dcd020(
    task: u32,
    identifier_flags: u32,
    initial_rate: f32,
    final_rate: f32
) -> u32 {
    const BASE_INIT_ID: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00ef97c4;
    const RANGE_START: usize = 0x30;
    const INITIAL_RATE_OFFSET: usize = 0x40;
    const FINAL_RATE_OFFSET: usize = 0x44;
    const RESOURCE_OFFSET: usize = 0x48;
    const COUNTER_OFFSET: usize = 0x4c;
    const FLAG_OFFSET: usize = 0x50;
    const IDENTIFIER_OFFSET: usize = 0x60;
    const CLEAR_FIELDS: [usize; 4] = [0x54, 0x58, 0x5c, 0x64];

    let _ = lf_checker_rt::callee_thiscall!(BASE_INIT_ID, u32, task);
    let count = ((identifier_flags & 0x0800_0000 != 0) as u32)
        + ((identifier_flags & 0x1000_0000 != 0) as u32);
    // SAFETY: the contract supplies a writable task object.
    unsafe {
        let bytes = task as *mut u8;
        (task as *mut u32).write(lf_checker_rt::relocated(TASK_VTABLE_VA));
        for index in 0..4 {
            (bytes.add(RANGE_START + index * 4) as *mut u32).write(u32::MAX);
        }
        (bytes.add(INITIAL_RATE_OFFSET) as *mut u32).write(initial_rate.to_bits());
        (bytes.add(FINAL_RATE_OFFSET) as *mut u32).write(final_rate.to_bits());
        (bytes.add(RESOURCE_OFFSET) as *mut u32).write(0);
        (bytes.add(COUNTER_OFFSET) as *mut u32).write(count);
        bytes.add(FLAG_OFFSET).write(0);
        for offset in CLEAR_FIELDS {
            (bytes.add(offset) as *mut u32).write(0);
        }
        (bytes.add(IDENTIFIER_OFFSET) as *mut u32).write(identifier_flags);
    }
    task
});
