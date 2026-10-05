// original: 0x0093e590 stream_publish_quad (proposed)

/// Publish a float quad plus level and mode bytes, then run the slot pass.
///
/// Copies four words from `q` to `OUT` (bit copies), stores the float
/// `level` at `LEVEL`, stores the low bytes of `m0`/`m1`/`m2` at
/// `MODE`/`AUX`/`ARMED_SRC`, zeroes `MODE_ARMED`, then calls the slot
/// pass (thiscall on the set at `SLOT_SET`) with the request callback
/// and a zero argument.
///
/// Original: 0x0093e590 (cdecl, five stack words; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e590(q: u32, level: u32, m0: u32, m1: u32, m2: u32) -> u32 {
    const OUT: u32 = 0x11D4E40;
    const LEVEL: u32 = 0x11A4FB0;
    const MODE: u32 = 0x11A4FB4;
    const AUX: u32 = 0x1036F0C;
    const MODE_ARMED: u32 = 0x11A4FB5;
    const ARMED_SRC: u32 = 0x11A4FB6;
    const SLOT_SET: u32 = 0x12E22A4;
    const SLOT_PASS: u32 = 1;
    const REQUEST_CB: u32 = 0x0093E480;
    unsafe {
        for i in 0..4u32 {
            let w = ((q + i * 4) as *const u32).read_unaligned();
            ((lf_checker_rt::relocated(OUT) + i * 4) as *mut u32).write_unaligned(w);
        }
        (lf_checker_rt::global::<u32>(LEVEL) as *mut u32).write_unaligned(level);
        (lf_checker_rt::global::<u8>(MODE) as *mut u8).write(m0 as u8);
        (lf_checker_rt::global::<u8>(AUX) as *mut u8).write(m1 as u8);
        (lf_checker_rt::global::<u8>(MODE_ARMED) as *mut u8).write(0);
        (lf_checker_rt::global::<u8>(ARMED_SRC) as *mut u8).write(m2 as u8);
        let set = (lf_checker_rt::global::<u32>(SLOT_SET) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(SLOT_PASS, u32, set, REQUEST_CB, 0u32)
    }
});
