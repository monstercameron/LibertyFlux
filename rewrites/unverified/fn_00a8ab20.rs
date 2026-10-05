// original: 0x00A8AB20 pool_latched_gate (proposed)

/// Run the latched three-stage gate, reporting the first stage that takes.
///
/// The low byte of `mode` selects the threshold (3 when zero, else 2) and
/// a flag (0 or 1) whose upper bytes are the caller's leftover stack slot
/// (only the low byte is meaningful and compared). The global latch byte
/// decides the first stage: when clear it is set and the level helper must
/// exceed the threshold, then the arm helper (low byte) must agree. Later
/// stages re-check: the second needs level above 2 plus the alt helper,
/// the third repeats the first pair. The first taking stage returns 1;
/// none taking returns 0. Only the low byte is set on the success path.
///
/// Original: stdcall, one stack word, low byte in AL. Three callees:
/// level (thiscall no args, three sites), arm (thiscall one arg, two
/// sites), alt (thiscall one arg, one site). The arm/alt argument is
/// compared low-byte-only (upper bytes are caller leftovers).
lf_checker_rt::export!(stdcall, rw_00A8AB20(mode: u32) -> u32 {
    unsafe {
        const LATCH: u32 = 0x12fb200;
        const SCOPE_A: u32 = 0x16dceb8;
        const SCOPE_B: u32 = 0x16dd2bc;
        const LEVEL: u32 = 1;
        const ARM: u32 = 2;
        const ALT: u32 = 3;
        let (threshold, flag) = if mode as u8 == 0 { (3u32, 0u32) } else { (2, 1) };
        let latch = lf_checker_rt::global::<u8>(LATCH);
        let was_clear = (latch as *const u8).read() == 0;
        (latch as *mut u8).write(was_clear as u8);
        if was_clear {
            let level: u32 = lf_checker_rt::callee_thiscall!(LEVEL, u32, SCOPE_A);
            if (level as i32) > threshold as i32 {
                let ok: u32 = lf_checker_rt::callee_thiscall!(ARM, u32, SCOPE_A, flag);
                if ok as u8 != 0 {
                    return 1;
                }
            }
        }
        let level: u32 = lf_checker_rt::callee_thiscall!(LEVEL, u32, SCOPE_B);
        if (level as i32) > 2 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(ALT, u32, SCOPE_A, flag);
            if ok as u8 != 0 {
                return 1;
            }
        }
        let level: u32 = lf_checker_rt::callee_thiscall!(LEVEL, u32, SCOPE_A);
        if (level as i32) > threshold as i32 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(ARM, u32, SCOPE_A, flag);
            if ok as u8 != 0 {
                return 1;
            }
        }
        0
    }
});
