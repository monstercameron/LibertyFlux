// original: 0x009e5030 CPedData::vf3

/// Allocate and set up a ped-data record, or null on failure.
///
/// Resolves the allocator (`thiscall` on the manager global, no stack
/// words); a null result returns 0. Otherwise calls setup (`thiscall`
/// on the new object with `(2, [a1 + 0xaa0], 1, [obj + 0x1c], 1)`,
/// the two middle words float bits), plants the vtable literal at +0,
/// caches the shared global at +0x80 and returns the object.
/// `stdcall`, one stack word.
lf_checker_rt::export!(stdcall, rw_009e5030(a1: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0167e2a0;
        const CACHED: u32 = 0x00ed7f40;
        const RATE_OFF: u32 = 0x1c;
        const REF_OFF: u32 = 0xaa0;
        const VTABLE: u32 = 0x00e98824;
        const ALLOC: u32 = 1;
        const SETUP: u32 = 2;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let obj = lf_checker_rt::callee_thiscall!(ALLOC, u32, mgr);
        if obj == 0 {
            return 0;
        }
        let f1 = ((obj + RATE_OFF) as *const u32).read_unaligned();
        let f2 = ((a1 + REF_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(SETUP, u32, obj, 2u32, f2, 1u32, f1, 1u32);
        (obj as *mut u32).write_unaligned(VTABLE);
        let c = lf_checker_rt::global::<u32>(CACHED).read();
        ((obj + 0x80) as *mut u32).write_unaligned(c);
        obj
    }
});
