// original: 0x00D3E810 task_simple_jump_launch_constructor
/// Initializes the base task and the jump-launch state. The first argument
/// is stored as a 16-bit value at offset 0x90; the optional second argument
/// is a four-word source record copied to offsets 0x80 through 0x8C. Its
/// contents are copied as words so the floating-point fields retain their
/// original bits.
lf_checker_rt::export!(thiscall, rw_d3e810(
    task: u32,
    launch_type: u32,
    source_record: u32,
) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const VTABLE: u32 = 0x00EE3DD4;
    const ONE_BITS: u32 = 0x3F800000;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, task);
    unsafe {
        (task as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((task + 0x14) as *mut u32).write_unaligned(0);
        ((task + 0x20) as *mut u32).write_unaligned(0);
        ((task + 0x24) as *mut u32).write_unaligned(0);
        ((task + 0x28) as *mut u32).write_unaligned(0);
        let flags = (task + 0x92) as *mut u8;
        flags.write(flags.read() & 0xC0);
        ((task + 0x30) as *mut u32).write_unaligned(0);
        ((task + 0x34) as *mut u32).write_unaligned(0);
        ((task + 0x38) as *mut u32).write_unaligned(u32::MAX);
        ((task + 0x90) as *mut u16).write_unaligned(launch_type as u16);
        ((task + 0x40) as *mut u32).write_unaligned(ONE_BITS);
        ((task + 0x44) as *mut u32).write_unaligned(0);
        ((task + 0x48) as *mut u32).write_unaligned(0);
        ((task + 0x50) as *mut u32).write_unaligned(0);
        ((task + 0x54) as *mut u32).write_unaligned(ONE_BITS);
        ((task + 0x58) as *mut u32).write_unaligned(0);
        ((task + 0x60) as *mut u32).write_unaligned(0);
        ((task + 0x64) as *mut u32).write_unaligned(0);
        ((task + 0x68) as *mut u32).write_unaligned(ONE_BITS);
        ((task + 0x78) as *mut u32).write_unaligned(0);
        ((task + 0x74) as *mut u32).write_unaligned(0);
        ((task + 0x70) as *mut u32).write_unaligned(0);
        if source_record != 0 {
            let source = source_record as *const u32;
            for index in 0..4 {
                ((task + 0x80 + index * 4) as *mut u32)
                    .write_unaligned(source.add(index as usize).read_unaligned());
            }
            flags.write(flags.read() | 0x20);
        }
    }
    task
});
