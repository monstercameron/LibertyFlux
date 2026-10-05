// original: 0x0093e610 stream_publish_quad3 (proposed)

/// Publish a float quad plus level, then run three slot passes.
///
/// Copies four words from `q` to `OUT` (bit copies) and stores the float
/// `level` at `LEVEL`, then calls the slot pass three times (thiscall):
/// on the set at `SLOT_SET_A` with the first callback, on the set at
/// `SLOT_SET_B` with the second, and on the set at `SLOT_SET_C` with the
/// third, each with a zero argument, setting `MODE_ARMED` and `MODE`
/// before the last one.
///
/// Original: 0x0093e610 (cdecl, two stack words; one callee, three sites).
lf_checker_rt::export!(cdecl, rw_0093e610(q: u32, level: u32) -> u32 {
    const OUT: u32 = 0x11D4E40;
    const LEVEL: u32 = 0x11A4FB0;
    const MODE_ARMED: u32 = 0x11A4FB5;
    const MODE: u32 = 0x11A4FB4;
    const SLOT_SET_A: u32 = 0x18B6F1C;
    const SLOT_SET_B: u32 = 0x18B6F10;
    const SLOT_SET_C: u32 = 0x12E22A4;
    const SLOT_PASS: u32 = 1;
    // File addresses of the request callbacks, relocated like the
    // original's pushed immediates (the worker's reloc pass covers them).
    const REQUEST_CB_A_FILE: u32 = 0x009407C0;
    const REQUEST_CB_B_FILE: u32 = 0x00940860;
    const REQUEST_CB_C_FILE: u32 = 0x0093E480;
    unsafe {
        for i in 0..4u32 {
            let w = ((q + i * 4) as *const u32).read_unaligned();
            ((lf_checker_rt::relocated(OUT) + i * 4) as *mut u32).write_unaligned(w);
        }
        (lf_checker_rt::global::<u32>(LEVEL) as *mut u32).write_unaligned(level);
        let a = (lf_checker_rt::global::<u32>(SLOT_SET_A) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            SLOT_PASS, u32, a, lf_checker_rt::relocated(REQUEST_CB_A_FILE), 0u32
        );
        let b = (lf_checker_rt::global::<u32>(SLOT_SET_B) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            SLOT_PASS, u32, b, lf_checker_rt::relocated(REQUEST_CB_B_FILE), 0u32
        );
        let c = (lf_checker_rt::global::<u32>(SLOT_SET_C) as *const u32).read_unaligned();
        (lf_checker_rt::global::<u8>(MODE_ARMED) as *mut u8).write(1);
        (lf_checker_rt::global::<u8>(MODE) as *mut u8).write(1);
        lf_checker_rt::callee_thiscall!(
            SLOT_PASS, u32, c, lf_checker_rt::relocated(REQUEST_CB_C_FILE), 0u32
        )
    }
});
