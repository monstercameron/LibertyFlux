// original: 0x009f85e0 Stat_ValidateForRegister
/// Bits of 1.0f, pushed as the float argument to the notify calls.
const ONE_BITS: u32 = 0x3F800000;

/// Max-clamp tunable setter (cdecl/2 -> void).
///
/// When the validity check passes, reads the current tunable value, clamps it
/// up to `limit` and stores it back. NaN on either side resolves to `limit`,
/// matching the original's comiss/ja pair.
export!(cdecl, rw_s18f5(id: u32, limit: f32) -> u32 {
    unsafe {
        let check: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        // The original tests only the low byte of the answer.
        if check(id) & 0xFF == 0 {
            return 0;
        }
        let get: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let cur = f32::from_bits(get(id));
        // comiss cur,limit + ja-keep: strictly-greater keeps cur, unordered
        // (NaN) takes limit — exactly `>` with the else branch below.
        let v = if cur > limit { cur } else { limit };
        let set: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        set(id, v.to_bits());
        0
    }
});
