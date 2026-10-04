// original: 0x009fb240 mode2_index_allowed
/// Dispatch table SW_T1_9FB240 for `rw_rs227_009fb240` (byte per input; 0 = accept, 1 = probe path).
const SW_T1_9FB240: [u8; 252] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
    0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
];
/// Dispatch table SW_T2_9FB240 for `rw_rs227_009fb240` (byte per input; 0 = accept, 1 = probe path).
const SW_T2_9FB240: [u8; 77] = [
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
];
/// Dispatch table SW_T3_9FB240 for `rw_rs227_009fb240` (byte per input; 0 = accept, 1 = probe path).
const SW_T3_9FB240: [u8; 206] = [
    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
];
/// Mode-gated index check.
///
/// Returns 1 when the mode global holds 2 and the index dispatches (through
/// three byte tables plus two fixed ranges) to accept. Otherwise probes once
/// through the shared predicate and returns 0; the probe's answer and the
/// trailing range check never accept, but the call itself is observable.
export!(cdecl, rw_rs227_009fb240(idx: u32) -> u8 {
    /// Mode value that enables the accept path.
    const ENABLED_MODE: u32 = 2;
    unsafe {
        if *global::<u32>(0x011D_6FD4) != ENABLED_MODE {
            return 0;
        }
        let probe: bool;
        if idx > 0x15E {
            if idx > 0x1CE {
                let t = idx.wrapping_sub(0x1D6);
                if t > 0xCD {
                    probe = true;
                } else {
                    probe = SW_T3_9FB240[t as usize] != 0;
                }
            } else if idx == 0x1CE {
                return 1;
            } else {
                let t = idx.wrapping_sub(0x160);
                if t > 0x4C {
                    probe = true;
                } else {
                    probe = SW_T2_9FB240[t as usize] != 0;
                }
            }
        } else if idx >= 0x14D {
            return 1;
        } else {
            let t = idx.wrapping_sub(1);
            if t > 0xFB {
                probe = true;
            } else {
                probe = SW_T1_9FB240[t as usize] != 0;
            }
        }
        if !probe {
            return 1;
        }
        let answer = callee_cdecl!(0, u32,);
        if (answer & 0xFF) != 0 {
            return 0;
        }
        if idx > 0x26F {
            return 0;
        }
        if idx >= 0x283 {
            return 1;
        }
        0
    }
});
