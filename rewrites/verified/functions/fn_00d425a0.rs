// original: 0x00d425a0 CTaskSimplePlayRandomAmbients::~CTaskSimplePlayRandomAmbients__deleting

/// Destroy the task, freeing its storage when the delete flag is set.
///
/// Runs the base destructor on `this`, then, only when bit 0 of `flags` is
/// set, returns the storage through the pool deallocator (whose object comes
/// from a static slot). Returns `this` on every path.
///
/// Original: thiscall, one stack word (flags), callee pops 4. The base
/// destructor takes no stack words; the deallocator takes the object as its
/// one stack word. Both are intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_00d425a0(this: u32, flags: u32) -> u32 {
    unsafe {
        const BASE_DTOR: u32 = 1;
        const FREE_CALLEE: u32 = 2;
        const POOL_SLOT: u32 = 0x0167e2a0;
        const DELETE_FLAG: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if flags & DELETE_FLAG != 0 {
            let pool = (lf_checker_rt::global::<u32>(POOL_SLOT) as *const u32).read_unaligned();
            let _: u32 = lf_checker_rt::callee_thiscall!(FREE_CALLEE, u32, pool, this);
        }
        this
    }
});
