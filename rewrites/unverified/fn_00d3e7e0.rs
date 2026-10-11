// original: 0x00D3E7E0 task_simple_jump_land_constructor
/// Initializes the base task, then stores two caller-provided words at
/// offsets 0x18 and 0x1C, clears offset 0x14, and installs this class's
/// vtable. It returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_d3e7e0(
    task: u32,
    landing_kind: u32,
    landing_data: u32,
) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const VTABLE: u32 = 0x00EE3E84;
    let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, task);
    unsafe {
        ((task + 0x18) as *mut u32).write_unaligned(landing_kind);
        ((task + 0x1C) as *mut u32).write_unaligned(landing_data);
        (task as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((task + 0x14) as *mut u32).write_unaligned(0);
    }
    task
});
