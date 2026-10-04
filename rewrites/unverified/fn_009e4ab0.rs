// original: 0x009e4ab0 task_kind_lookup (proposed)

/// Map a kind word through a 78-entry 0/1 table, default 1.
///
/// Reads the word at `p`, subtracts 0x12d and, when the result exceeds
/// 0x4d, returns 1. Otherwise returns the table byte at that index (the
/// original dispatches through a two-target jump table whose arms
/// return 0 and 1, so the result equals the byte). Only `al` is
/// defined on return. `cdecl`, one stack word.
lf_checker_rt::export!(cdecl, rw_009e4ab0(p: u32) -> u32 {
    unsafe {
        const BIAS: u32 = 0x12d;
        const LIMIT: u32 = 0x4d;
        const TABLE: u32 = 0x009e4adc;
        let v = ((p) as *const u32).read_unaligned().wrapping_sub(BIAS);
        if v > LIMIT {
            return 1;
        }
        ((lf_checker_rt::relocated(TABLE) + v) as *const u8).read() as u32
    }
});
