// original: 0x008b4970 eleven_gate_readiness_check
use lf_checker_rt::{callee_cdecl, export, global};

/// Eleven-gate readiness check over provider records.
///
/// Each of eleven rounds asks the provider (always with selector 1) for a
/// record and xors two bytes at a round-specific offset pair inside it. A
/// round passes when the xor is at most 0x7f; the first failing round ends
/// the check early with 0, so the number of provider calls is the number of
/// rounds reached. When every round passes the result is 1.
///
/// The stored byte is then consulted: a clear stored byte forces the result
/// to 1 regardless. Otherwise the check result stands.
export!(cdecl, rw_008b4970() -> u32 {
    /// Stored override byte: clear forces a passing result.
    const OVERRIDE: u32 = 0x0116_0B85;
    /// (first, second) byte offsets xored in each of the eleven rounds.
    const PAIRS: [(u32, u32); 11] = [
        (0x2aae, 0x2aac),
        (0x2a9e, 0x2a9c),
        (0x2abe, 0x2abc),
        (0x2ace, 0x2acc),
        (0x2b6e, 0x2b6c),
        (0x2b8e, 0x2b8c),
        (0x2b9e, 0x2b9c),
        (0x2b7e, 0x2b7c),
        (0x2bae, 0x2bac),
        (0x2bde, 0x2bdc),
        (0x2bce, 0x2bcc),
    ];
    unsafe {
        let mut ok = 1u32;
        for (first, second) in PAIRS {
            let record = callee_cdecl!(1, u32, 1);
            let a = (record.wrapping_add(first) as *const u8).read();
            let b = (record.wrapping_add(second) as *const u8).read();
            if (a ^ b) > 0x7f {
                ok = 0;
                break;
            }
        }
        if global::<u8>(OVERRIDE).read() == 0 || ok != 0 {
            1
        } else {
            0
        }
    }
});
