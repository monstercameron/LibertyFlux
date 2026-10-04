// original: 0x009e4ed0 CPedData::vf4

/// Build one of three ped-data helpers selected by kind bytes.
///
/// When `[a1 + 0x219]` is nonzero, allocates (`thiscall` on the manager
/// global, no stack words) and builds variant A (`thiscall` on the new
/// object with `a2`). Otherwise, when `[a1 + 0xa60]` equals 2, builds
/// variant B with `(a2, 0, 0, a3)`; any other value builds variant C
/// with `(a2, 0, 10, 0)`. A null allocation returns 0; otherwise the
/// builder's answer is returned. `stdcall`, three stack words.
lf_checker_rt::export!(stdcall, rw_009e4ed0(a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0167e2a0;
        const KIND_A: u32 = 0x219;
        const KIND_B: u32 = 0xa60;
        const ALLOC: u32 = 1;
        const MAKE_A: u32 = 2;
        const MAKE_B: u32 = 3;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        if ((a1 + KIND_A) as *const u8).read() != 0 {
            let r = lf_checker_rt::callee_thiscall!(ALLOC, u32, mgr);
            if r == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(MAKE_A, u32, r, a2);
        }
        if ((a1 + KIND_B) as *const u8).read() == 2 {
            let r = lf_checker_rt::callee_thiscall!(ALLOC, u32, mgr);
            if r == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(MAKE_B, u32, r, a2, 0u32, 0u32, a3);
        }
        let r = lf_checker_rt::callee_thiscall!(ALLOC, u32, mgr);
        if r == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(MAKE_B, u32, r, a2, 0u32, 10u32, 0u32)
    }
});
