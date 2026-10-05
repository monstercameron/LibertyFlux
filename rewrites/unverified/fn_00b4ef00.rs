// original: 0x00b4ef00 CDummyPed::vf47

/// Build the dummy ped's task handle from five arguments, or clear it.
///
/// The pool manager is fetched (callee 1, thiscall on the pool object read
/// from game address `PED_POOL`). When that yields null, the finish hook
/// (callee 3, thiscall on null with `e`) runs, the handle at `this + 0x6C`
/// is cleared and null is returned. Otherwise a handle is created (callee
/// 2, thiscall on the manager with `this`, kind 2 and the four arguments
/// `a` to `d`), finished (callee 3, thiscall on the handle with `e`),
/// stored to the handle slot and returned.
///
/// Original: 0x00b4ef00 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00b4ef00(this: u32, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 1;
        const CREATE: u32 = 2;
        const FINISH: u32 = 3;
        const PED_POOL: u32 = 0x018b6b98;
        const HANDLE: u32 = 0x6c;
        const KIND: u32 = 2;
        let pool = lf_checker_rt::global::<u32>(PED_POOL).read_unaligned();
        let mgr = lf_checker_rt::callee_thiscall!(MANAGER, u32, pool);
        if mgr == 0 {
            lf_checker_rt::callee_thiscall!(FINISH, u32, 0, e);
            ((this + HANDLE) as *mut u32).write_unaligned(0);
            return 0;
        }
        let h = lf_checker_rt::callee_thiscall!(CREATE, u32, mgr, this, KIND, a, b, c, d);
        lf_checker_rt::callee_thiscall!(FINISH, u32, h, e);
        ((this + HANDLE) as *mut u32).write_unaligned(h);
        h
    }
});
