// original: 0x0093e4f0 stream_publish_pair (proposed)

/// Publish two float quads and trigger the slot pass.
///
/// Copies four words from `q0` to `OUT_A` and four from `q1` to `OUT_B`
/// (bit copies, no arithmetic), stores the float `level` at `LEVEL`,
/// stores the low byte of `mode` at `MODE` and zeroes `MODE_ARMED`, then
/// calls the slot pass (thiscall on the set at `SLOT_SET`) with the
/// request callback and a zero argument. Note the order: level is the
/// third word, mode the fourth.
///
/// Original: 0x0093e4f0 (cdecl, four stack words; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e4f0(q0: u32, q1: u32, level: u32, mode: u32) -> u32 {
    const OUT_A: u32 = 0x11D4E50;
    const OUT_B: u32 = 0x11D4E60;
    const LEVEL: u32 = 0x11A4FB8;
    const MODE: u32 = 0x11A4FB4;
    const MODE_ARMED: u32 = 0x11A4FB5;
    const SLOT_SET: u32 = 0x12E22A4;
    const SLOT_PASS: u32 = 1;
    // File address of the request callback, relocated like the original's
    // pushed immediate (the worker's reloc pass covers it).
    const REQUEST_CB_FILE: u32 = 0x0093E440;
    unsafe {
        for i in 0..4u32 {
            let w = ((q0 + i * 4) as *const u32).read_unaligned();
            ((lf_checker_rt::relocated(OUT_A) + i * 4) as *mut u32).write_unaligned(w);
        }
        for i in 0..4u32 {
            let w = ((q1 + i * 4) as *const u32).read_unaligned();
            ((lf_checker_rt::relocated(OUT_B) + i * 4) as *mut u32).write_unaligned(w);
        }
        (lf_checker_rt::global::<u32>(LEVEL) as *mut u32).write_unaligned(level);
        (lf_checker_rt::global::<u8>(MODE) as *mut u8).write(mode as u8);
        (lf_checker_rt::global::<u8>(MODE_ARMED) as *mut u8).write(0);
        let set = (lf_checker_rt::global::<u32>(SLOT_SET) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            SLOT_PASS, u32, set, lf_checker_rt::relocated(REQUEST_CB_FILE), 0u32
        )
    }
});
