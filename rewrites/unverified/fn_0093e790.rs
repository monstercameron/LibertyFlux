// original: 0x0093e790 stream_publish_quad2 (proposed)

/// Publish a float quad plus level, then run two slot passes.
///
/// Copies four words from `q` to `OUT` (bit copies) and stores the float
/// `level` at `LEVEL`, then calls the slot pass twice (thiscall): first
/// on the set at `SLOT_SET_A` with the first callback, then on the set
/// at `SLOT_SET_B` with the second, both with a zero argument.
///
/// Original: 0x0093e790 (cdecl, two stack words; one callee, two sites).
lf_checker_rt::export!(cdecl, rw_0093e790(q: u32, level: u32) -> u32 {
    const OUT: u32 = 0x11D4E40;
    const LEVEL: u32 = 0x11A4FB0;
    const SLOT_SET_A: u32 = 0x18B6F1C;
    const SLOT_SET_B: u32 = 0x18B6F10;
    const SLOT_PASS: u32 = 1;
    // File addresses of the request callbacks, relocated like the
    // original's pushed immediates (the worker's reloc pass covers them).
    const REQUEST_CB_A_FILE: u32 = 0x00941160;
    const REQUEST_CB_B_FILE: u32 = 0x00940900;
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
        )
    }
});
