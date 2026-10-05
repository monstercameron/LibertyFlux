// original: 0x008A9E50 audio_state_init (proposed)

/// Initialise the shared audio state block and create two handles.
///
/// Zeroes the flag byte at `0x115f850` and the dwords at `0x115f854`,
/// `0x115f85c`, `0x115f84c` and `0x115f848`, runs the subsystem prepare
/// (`0x8accf0`, cdecl, no arguments), then stores the results of two handle
/// creations (`0x403c70`, cdecl, each called with 1) into `0x115f858` and
/// `0x115f860`. Returns 1 in `al`. Original is cdecl with no stack words
/// (plain `ret`).
lf_checker_rt::export!(cdecl, rw_008A9E50() -> u32 {
    const PREPARE: u32 = 1;
    const CREATE: u32 = 2;
    unsafe {
        (lf_checker_rt::global::<u8>(0x0115_f850) as *mut u8).write(0);
        for a in [0x0115_f854u32, 0x0115_f85c, 0x0115_f84c, 0x0115_f848] {
            (lf_checker_rt::global::<u32>(a) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_cdecl!(PREPARE, u32,);
        let h1: u32 = lf_checker_rt::callee_cdecl!(CREATE, u32, 1u32);
        (lf_checker_rt::global::<u32>(0x0115_f858) as *mut u32).write_unaligned(h1);
        let h2: u32 = lf_checker_rt::callee_cdecl!(CREATE, u32, 1u32);
        (lf_checker_rt::global::<u32>(0x0115_f860) as *mut u32).write_unaligned(h2);
        1
    }
});
