// original: 0x008F98F0 stream_slot_ready_select
/// Test slot readiness on the pool selected by a flag byte.
///
/// Forwards the index to the readiness routine on the first global
/// pool when the flag byte is nonzero, else on the second pool, and
/// returns its boolean result. Cdecl, two stack arguments.
export!(cdecl, rw_008f98f0(flag: u32, idx: u32) -> u32 {
    unsafe {
        const POOL_A: u32 = 0x118e7c0;
        const POOL_B: u32 = 0x118e7c8;
        let this = if (flag as u8) == 0 { relocated(POOL_B) } else { relocated(POOL_A) };
        callee_thiscall!(1, u32, this, idx)
    }
});
