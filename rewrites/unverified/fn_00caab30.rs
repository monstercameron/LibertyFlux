// original: 0x00CAAB30 ped_pair_ready (proposed)

/// Decide whether a ped pair is ready from a flag and two lookups.
///
/// Returns 1 when the flag byte at `arg+0xA60` equals 2. Otherwise resolves
/// two records from `arg+0x2B0`: a null second record also means ready (1),
/// while a null first record means not ready (0). With both records present,
/// looks each one's `+0x18` word up and returns 1 unless the first lookup's
/// `+0x0C` word equals 1 while the second's differs from 1. Returns the
/// decision in al.
///
/// Original: 0x00CAAB30 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00caab30(arg: u32) -> u32 {
    unsafe {
        const RESOLVER_A: u32 = 1;
        const RESOLVER_B: u32 = 2;
        const LOOKUP_A: u32 = 3;
        const LOOKUP_B: u32 = 4;
        const FLAG: u32 = 0xA60;
        const READY_FLAG: u8 = 2;
        const INNER: u32 = 0x2B0;
        const KEY: u32 = 0x18;
        const STATE: u32 = 0x0C;
        const LIVE_STATE: u32 = 1;
        let flag = (arg.wrapping_add(FLAG) as *const u8).read();
        if flag == READY_FLAG {
            return 1;
        }
        let first = lf_checker_rt::callee_thiscall!(RESOLVER_A, u32, arg.wrapping_add(INNER));
        let second = lf_checker_rt::callee_thiscall!(RESOLVER_B, u32, arg.wrapping_add(INNER));
        if second == 0 {
            return 1;
        }
        if first == 0 {
            return 0;
        }
        let key_a = (first.wrapping_add(KEY) as *const u32).read_unaligned();
        let hit_a = lf_checker_rt::callee_cdecl!(LOOKUP_A, u32, key_a);
        let state_a = (hit_a.wrapping_add(STATE) as *const u32).read_unaligned();
        if state_a != LIVE_STATE {
            return 1;
        }
        let key_b = (second.wrapping_add(KEY) as *const u32).read_unaligned();
        let hit_b = lf_checker_rt::callee_cdecl!(LOOKUP_B, u32, key_b);
        let state_b = (hit_b.wrapping_add(STATE) as *const u32).read_unaligned();
        if state_b == LIVE_STATE {
            return 1;
        }
        0
    }
});
