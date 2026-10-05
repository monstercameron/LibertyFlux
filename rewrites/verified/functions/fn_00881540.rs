// original: 0x00881540 stream_mgr_slot_init (proposed)
/// Initialise one manager slot and clear its two cache lines.
///
/// The slot's record address is `100 * slot + bias`, where the bias comes
/// from `mgr+0x58`; the record is handed (in `ecx`, with `mgr` as the stack
/// argument) to the endpoint initialiser (intercepted callee 1, thiscall,
/// one argument). Then the slot's limit word is set to `LIMIT` (`0x5208`)
/// in the table at `mgr+0x60` and its companion word cleared in the table at
/// `mgr+0x64`, and both cache blocks (at `mgr+0x6c` and `mgr+0x68`) have the
/// four words at offsets `0x50`, `0x60`, `+0x70`, `+0x80` past line `line`
/// cleared, where `line = (slot & 3) + 36 * (slot >> 2)`.
///
/// Original: thiscall, one stack argument, callee cleans 4, no return value.
lf_checker_rt::export!(thiscall, rw_00881540(mgr: u32, slot: u32) -> u32 {
    unsafe {
        const RECORD_STRIDE: u32 = 100;
        const LIMIT: u32 = 0x5208;
        const BIAS: u32 = 0x58;
        const LIMIT_TABLE: u32 = 0x60;
        const COMPANION_TABLE: u32 = 0x64;
        const CACHE_A: u32 = 0x6c;
        const CACHE_B: u32 = 0x68;
        const INIT_CALLEE: u32 = 1;
        let bias = ((mgr + BIAS) as *const u32).read_unaligned();
        let record = slot.wrapping_mul(RECORD_STRIDE).wrapping_add(bias);
        let _done: u32 = lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, record, mgr);
        let limits = ((mgr + LIMIT_TABLE) as *const u32).read_unaligned();
        ((limits.wrapping_add(slot.wrapping_mul(4))) as *mut u32).write_unaligned(LIMIT);
        let companions = ((mgr + COMPANION_TABLE) as *const u32).read_unaligned();
        ((companions.wrapping_add(slot.wrapping_mul(4))) as *mut u32).write_unaligned(0);
        // line = (slot & 3) + 4 * (9 * (slot >> 2)).
        let line = (slot & 3).wrapping_add((slot >> 2).wrapping_mul(9).wrapping_mul(4));
        let base = line.wrapping_mul(4);
        for cache_off in [CACHE_A, CACHE_B] {
            let cache = ((mgr + cache_off) as *const u32).read_unaligned();
            for word in [0x60u32, 0x50, 0x80, 0x70] {
                (cache.wrapping_add(base).wrapping_add(word) as *mut u32).write_unaligned(0);
            }
        }
        0
    }
});
