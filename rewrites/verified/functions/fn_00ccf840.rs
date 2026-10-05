// original: 0x00ccf840 CTaskComplexMelee::vf0
/// Destroy the complex melee task, freeing it when the flag asks.
///
/// Runs the destructor (thiscall on `this`), then, when the low bit of the
/// stack argument is set, hands `this` to the task allocator's free helper
/// (thiscall on the global manager pointer). Returns `this`. Thiscall.
export!(thiscall, rw_00ccf840(this: u32, flags: u32) -> u32 {
    unsafe {
        const MGR_G: u32 = 0x0167e2a0;
        const FREE_BIT: u32 = 1;
        let _: u32 = callee_thiscall!(1, u32, this);
        if flags & FREE_BIT != 0 {
            let mgr = *global::<u32>(MGR_G);
            let _: u32 = callee_thiscall!(2, u32, mgr, this);
        }
        this
    }
});
