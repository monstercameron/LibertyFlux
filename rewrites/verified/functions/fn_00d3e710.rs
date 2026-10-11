// original: 0x00D3E710 task_simple_jump_in_air_constructor
/// Initializes a jump-in-air task from six stack arguments. The first two
/// words are copied to offsets 0x18 and 0x1C, the next is a float bit pattern
/// for offset 0x20, and the next two bytes select low flag bits. A non-null
/// source record copies four words to 0x60 through 0x6C and sets flag bit 3.
/// The final word is a pointer to that record; a global word supplies offset
/// 0x48. Float data is copied without arithmetic.
lf_checker_rt::export!(thiscall, rw_d3e710(
    task: u32,
    owner: u32,
    task_kind: u32,
    duration_bits: u32,
    first_flag: u32,
    second_flag: u32,
    source_record: u32,
) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const GLOBAL_WORD: u32 = 0x011735B4;
    const VTABLE: u32 = 0x00EE3E2C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, task);
    unsafe {
        ((task + 0x18) as *mut u32).write_unaligned(owner);
        ((task + 0x1C) as *mut u32).write_unaligned(task_kind);
        (task as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((task + 0x14) as *mut u32).write_unaligned(0);
        ((task + 0x20) as *mut u32).write_unaligned(duration_bits);
        ((task + 0x30) as *mut u32).write_unaligned(0);
        ((task + 0x34) as *mut u32).write_unaligned(0);
        ((task + 0x38) as *mut u32).write_unaligned(0);
        let mut flags = ((second_flag as u8 & 1) << 1) | (first_flag as u8 & 1);
        ((task + 0x40) as *mut u32).write_unaligned(0);
        ((task + 0x44) as *mut u32).write_unaligned(0);
        ((task + 0x48) as *mut u32).write_unaligned(0);
        ((task + 0x4C) as *mut u32).write_unaligned(0);
        ((task + 0x50) as *mut u16).write_unaligned(0);
        let high_flags = (task + 0x70) as *mut u8;
        flags = (flags << 1) | (high_flags.read() & 0xF0);
        high_flags.write(flags);
        if source_record != 0 {
            let source = source_record as *const u32;
            for index in 0..4 {
                ((task + 0x60 + index * 4) as *mut u32)
                    .write_unaligned(source.add(index as usize).read_unaligned());
            }
            high_flags.write(high_flags.read() | 0x08);
        }
        let shared_state = lf_checker_rt::global::<u32>(GLOBAL_WORD).read_unaligned();
        ((task + 0x48) as *mut u32).write_unaligned(shared_state);
        ((task + 0x4C) as *mut u32).write_unaligned(500);
        ((task + 0x50) as *mut u8).write(1);
    }
    task
});
