// original: 0x00a3ab50 vehicle_indexed_release (proposed)

/// Release the table slot selected by the object's index field.
///
/// Returns 0 (the contract fixes entry EAX to 0, which the original
/// preserves on this path) when the head word at `+0x80` is zero, and -1
/// when the index at `+0x84` is -1. Otherwise hands `TABLE + index*48` to
/// the release callee (id 1) and returns its answer. Thiscall, no stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_00a3ab50(obj: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x80;
        const INDEX: u32 = 0x84;
        const TABLE: u32 = 0x012E_1EE0;
        const NO_INDEX: u32 = 0xFFFF_FFFF;
        const RELEASE: u32 = 1;
        if core::ptr::read_unaligned((obj + HEAD) as *const u32) == 0 {
            return 0;
        }
        let idx = core::ptr::read_unaligned((obj + INDEX) as *const u32);
        if idx == NO_INDEX {
            return idx;
        }
        let slot = lf_checker_rt::relocated(TABLE.wrapping_add(idx.wrapping_mul(3).wrapping_shl(4)));
        lf_checker_rt::callee_cdecl!(RELEASE, u32, slot)
    }
});
