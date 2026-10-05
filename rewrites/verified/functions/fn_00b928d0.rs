// original: 0x00b928d0 NativeImpl_TEXT_RESOLVE_SUBSTRING_2

/// Resolves two text keys and prints them through a 16-argument draw call.
///
/// Resolves `key0` and `key1` through `LOOKUP` (with flag 0) and probes the
/// state block from `PROBE`: the 14-argument `SETUP` call (stdcall) runs
/// unless the probe byte is clear while the two globals `G2`/`G3` are set
/// and differ. When the `DONE` flag byte is set, reads a selector byte
/// through `SEL` (called with 1) and issues the 16-argument `DRAW` call.
/// Always raises `DONE`, then notifies through `NOTIFY_A`/`NOTIFY_B`
/// (thiscall on `OBJ` with `key0`/`key1`). Returns the last notify result;
/// the contract does not compare it.
///
/// The spilled `G1` words are frame scratch, but `G1` itself is verified
/// through the draw call's arguments whenever it fires.
///
/// Original: 0x00B928D0 (cdecl, four stack words, no compared return).
lf_checker_rt::export!(cdecl, rw_00b928d0(key0: u32, key1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const PROBE: u32 = 2;
        const SETUP: u32 = 3;
        const SEL: u32 = 4;
        const DRAW: u32 = 5;
        const NOTIFY_A: u32 = 6;
        const NOTIFY_B: u32 = 7;
        const OBJ: u32 = 0x01033130;
        const G1: u32 = 0x0116C24C;
        const G2: u32 = 0x011E6248;
        const DONE: u32 = 0x011E622E;
        const PROBE_OFF: u32 = 0x99;
        const SEL_OFF: u32 = 0x328C;

        let r1: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key0, 0);
        let r2: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key1, 0);
        let g1 = lf_checker_rt::global::<u32>(G1).read();
        let pb: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32,);
        let mut setup_done = true;
        if (pb.wrapping_add(PROBE_OFF) as *const u8).read() == 0 {
            let g2 = lf_checker_rt::global::<u32>(G2).read();
            if g2 != 0 && g2 != lf_checker_rt::global::<u32>(G2.wrapping_add(4)).read() {
                setup_done = false;
            }
        }
        if setup_done {
            let _: u32 = lf_checker_rt::callee_stdcall!(
                SETUP, u32, r1, 0, a2, 0, 0, r2, 0, 0, 0, 0, a3, 0, 1, 0xFFFF_FFFF
            );
        }
        if setup_done && lf_checker_rt::global::<u8>(DONE).read() != 0 {
            let s: u32 = lf_checker_rt::callee_cdecl!(SEL, u32, 1);
            let sel = (s.wrapping_add(SEL_OFF) as *const u8).read() as u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                DRAW, u32, r1, g1, r2, g1, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF,
                0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0, sel, key0,
                key1
            );
        }
        lf_checker_rt::global::<u8>(DONE).write(1);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            NOTIFY_A, u32, lf_checker_rt::relocated(OBJ), key0
        );
        let out: u32 = lf_checker_rt::callee_thiscall!(
            NOTIFY_B, u32, lf_checker_rt::relocated(OBJ), key1
        );
        out
    }
});
