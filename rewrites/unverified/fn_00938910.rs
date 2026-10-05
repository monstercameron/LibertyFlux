// original: 0x00938910 stream_state_dispatch (proposed)

/// Dispatch on the streaming state past the readiness gates.
///
/// Fails through the shared report (answering its leftover with AL
/// cleared) when the probe is quiet, the scan is empty, the pool is busy,
/// the override names any mode but 5 or 2, verification complains, or the
/// final poll is set. A set flag late answers the verify leftover
/// directly. Otherwise the state selects an arm: 1 and 4 answer 0,
/// anything above 6 answers the state with AL cleared, the rest 1.
/// The switch arms carry no leftover: the table index overwrites EAX.
lf_checker_rt::export!(cdecl, rw_00938910() -> u32 {
    unsafe {
        const PROBE: u32 = 1;
        const FAIL: u32 = 2;
        const SCAN: u32 = 3;
        const VERIFY: u32 = 4;
        const POLL: u32 = 5;
        const PROBE_OBJ: u32 = 0x115D9A0;
        const POOL: u32 = 0x11D6FA0;
        const OVERRIDE: u32 = 0x1161518;
        const MODE: u32 = 0x11A4EF4;
        const FLAG: u32 = 0x11609F6;
        const LOW_MASK: u32 = 0xFFFF_FF00;
        let g = |va: u32| -> u32 {
            lf_checker_rt::global::<u32>(va).read_unaligned()
        };
        let gb = |va: u32| -> u8 {
            lf_checker_rt::global::<u8>(va).read()
        };
        let t: u32 = lf_checker_rt::callee_thiscall!(
            PROBE,
            u32,
            lf_checker_rt::relocated(PROBE_OBJ)
        );
        if (t & 0xFF) == 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(FAIL, u32, 1);
            return r & LOW_MASK;
        }
        let s: u32 = lf_checker_rt::callee_cdecl!(SCAN, u32,);
        if (s & 0xFF) == 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(FAIL, u32, 1);
            return r & LOW_MASK;
        }
        if g(POOL) != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(FAIL, u32, 1);
            return r & LOW_MASK;
        }
        if gb(OVERRIDE) != 0 {
            let v = g(MODE);
            if v != 5 && v != 2 {
                let r: u32 = lf_checker_rt::callee_cdecl!(FAIL, u32, 1);
                return r & LOW_MASK;
            }
        }
        let k: u32 = lf_checker_rt::callee_cdecl!(VERIFY, u32,);
        if (k & 0xFF) != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(FAIL, u32, 1);
            return r & LOW_MASK;
        }
        if gb(FLAG) != 0 {
            return k & LOW_MASK;
        }
        let u: u32 = lf_checker_rt::callee_cdecl!(POLL, u32,);
        if (u & 0xFF) != 0 {
            let r: u32 = lf_checker_rt::callee_cdecl!(FAIL, u32, 1);
            return r & LOW_MASK;
        }
        // Decoded from the in-code jump table (index at code+0x938994,
        // targets at code+0x93898C): the index byte is zero-extended into
        // EAX first, so the success arms answer exactly 1 and the fail
        // arms exactly 0, whatever the poll returned. Above 6 the state
        // itself is still in EAX, so AL is cleared on it. (The table
        // lives in executable code, which a rewrite may not read, so the
        // decoded arms are matched directly.)
        let v2 = g(MODE);
        if v2 > 6 {
            return v2 & LOW_MASK;
        }
        if v2 == 1 || v2 == 4 {
            return 0;
        }
        1
    }
});
