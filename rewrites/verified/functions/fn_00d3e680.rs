// original: 0x00D3E680 CTaskComplexJump::CTaskComplexJump
/// Initializes the jump-task fields after the base task constructor. The
/// optional source record is four words copied to offsets 0x20 through 0x2C;
/// its three floating-point words are preserved bit for bit. The two stack
/// arguments are a 16-bit type value and the optional record pointer.
lf_checker_rt::export!(thiscall, rw_d3e680(
    task: u32,
    task_type: u32,
    source_record: u32,
) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const VTABLE: u32 = 0x00EE3D7C;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, task);
    unsafe {
        (task as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((task + 0x30) as *mut u32).write_unaligned(0);
        ((task + 0x34) as *mut u32).write_unaligned(0);
        ((task + 0x38) as *mut u32).write_unaligned(0);
        ((task + 0x3C) as *mut u16).write_unaligned(task_type as u16);
        ((task + 0x40) as *mut u32).write_unaligned(0);
        ((task + 0x44) as *mut u32).write_unaligned(0);
        ((task + 0x48) as *mut u32).write_unaligned(0);
        let flags = (task + 0x5C) as *mut u8;
        flags.write(flags.read() & 0xF0);
        ((task + 0x50) as *mut u32).write_unaligned(0);
        ((task + 0x54) as *mut u32).write_unaligned(0);
        ((task + 0x58) as *mut u32).write_unaligned(u32::MAX);
        if source_record != 0 {
            let source = source_record as *const u32;
            for index in 0..4 {
                ((task + 0x20 + index * 4) as *mut u32)
                    .write_unaligned(source.add(index as usize).read_unaligned());
            }
            flags.write(flags.read() | 0x08);
        }
    }
    task
});
