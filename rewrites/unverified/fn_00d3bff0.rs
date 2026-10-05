// original: 0x00d3bff0 CDummyTask_Wander::vf3

/// Wander dummy task operation dispatcher (vtable slot 3): six operations
/// selected by `op` on the task object `task` with a peer object `stream`.
///
/// Layout of `task`: `+0x10` flag byte, `+0x14` integer. `stream` is only
/// passed through to the callees.
///
/// Operations: 0 reads the flag and the integer from `stream` into the task
/// (the integer only when the flag is non-zero; the original answers that
/// integer through its own incoming `op` slot, clobbering one caller-stack
/// word, which a Rust rewrite cannot address, so the proof leaves the stack
/// check off and verifies the value through the `+0x14` store instead);
/// 1 reads the flag and, when
/// non-zero, writes an immediate pair; 2 does nothing; 3 writes the flag
/// byte and, when non-zero, the integer into `stream`; 4 formats the task's
/// state through the log callee, resolving the integer through the format
/// callee when the flag is non-zero; 5 returns 8 when the flag is non-zero
/// and 1 otherwise. All paths except operation 5 return 0.
///
/// Original: 0x00d3bff0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00d3bff0(task: u32, op: u32, stream: u32) -> u32 {
    unsafe {
        const F_FLAG: u32 = 0x10;
        const F_INT: u32 = 0x14;
        const INT_WIDTH: u32 = 7;
        const LOG_FILE: u32 = 0x0198b570;
        const FMT_HEAD: u32 = 0x00ee35d4;
        const FMT_SEL_A: u32 = 0x00ee35e8;
        const FMT_SEL_B: u32 = 0x00ee35f0;
        const FMT_SEL: u32 = 0x00ee35f8;
        const FMT_INT: u32 = 0x00ee360c;

        const C_READ_FLAG: u32 = 1;
        const C_READ_INT: u32 = 2;
        const C_WRITE_IMM: u32 = 3;
        const C_WRITE_FLAG: u32 = 4;
        const C_WRITE_INT: u32 = 5;
        const C_LOG4: u32 = 6;
        const C_LOG5: u32 = 7;
        const C_FORMAT: u32 = 8;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        match op {
            0 => {
                lf_checker_rt::callee_thiscall!(C_READ_FLAG, u32, stream, task + F_FLAG);
                if rd8(task + F_FLAG) == 0 {
                    return 0;
                }
                let mut slot: u32 = 0;
                lf_checker_rt::callee_thiscall!(
                    C_READ_INT,
                    u32,
                    stream,
                    &mut slot as *mut u32 as u32,
                    INT_WIDTH
                );
                wr32(task + F_INT, slot);
                0
            }
            1 => {
                lf_checker_rt::callee_thiscall!(C_READ_FLAG, u32, stream, task + F_FLAG);
                if rd8(task + F_FLAG) == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(C_WRITE_IMM, u32, stream, INT_WIDTH, 1);
                0
            }
            3 => {
                lf_checker_rt::callee_thiscall!(
                    C_WRITE_FLAG,
                    u32,
                    stream,
                    rd8(task + F_FLAG) as u32
                );
                if rd8(task + F_FLAG) == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    C_WRITE_INT,
                    u32,
                    stream,
                    rd32(task + F_INT),
                    INT_WIDTH
                );
                0
            }
            4 => {
                let file = lf_checker_rt::relocated(LOG_FILE);
                lf_checker_rt::callee_cdecl!(
                    C_LOG4,
                    u32,
                    file,
                    0,
                    0,
                    lf_checker_rt::relocated(FMT_HEAD)
                );
                let sel = if rd8(task + F_FLAG) == 0 { FMT_SEL_B } else { FMT_SEL_A };
                lf_checker_rt::callee_cdecl!(
                    C_LOG5,
                    u32,
                    file,
                    0,
                    1,
                    lf_checker_rt::relocated(FMT_SEL),
                    lf_checker_rt::relocated(sel)
                );
                if rd8(task + F_FLAG) == 0 {
                    return 0;
                }
                let v: u32 =
                    lf_checker_rt::callee_cdecl!(C_FORMAT, u32, rd32(task + F_INT));
                lf_checker_rt::callee_cdecl!(
                    C_LOG5,
                    u32,
                    file,
                    0,
                    1,
                    lf_checker_rt::relocated(FMT_INT),
                    v
                );
                0
            }
            5 => {
                if rd8(task + F_FLAG) == 0 {
                    1
                } else {
                    8
                }
            }
            _ => 0,
        }
    }
});
