// original: 0x00c80c50 scenario_slot_acquire (proposed)

/// Acquire a scenario slot for record `a1`, resolving the slot id first.
///
/// Calls `c1([a1+0x20]+0x30)`; a non-zero low byte means the record is already
/// placed and the function returns 0. Otherwise the wanted id is `a2`, except
/// that -1 resolves through `c2(a1)`, whose -1 means failure (return 0). On
/// success calls `c3(id, a1)` and returns 1. Only AL carries the result.
///
/// Original: cdecl, two stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c80c50(a1: u32, a2: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x20;
        const KEY_BIAS: u32 = 0x30;
        const UNRESOLVED: u32 = 0xffff_ffff;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        let esi = a1;
        let key = ((esi + KEY_OFF) as *const u32).read_unaligned().wrapping_add(KEY_BIAS);
        let placed: u32 = lf_checker_rt::callee_cdecl!(C1, u32, key);
        if (placed & 0xff) != 0 {
            return 0;
        }
        let mut id = a2;
        if id == UNRESOLVED {
            id = lf_checker_rt::callee_cdecl!(C2, u32, esi);
            if id == UNRESOLVED {
                return 0;
            }
        }
        lf_checker_rt::callee_cdecl!(C3, u32, id, esi);
        1
    }
});
