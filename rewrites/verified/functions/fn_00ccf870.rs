// original: 0x00ccf870 CTaskSimpleMeleeActionResult::vf0
/// Destroy the melee action-result task, freeing it when the flag asks.
///
/// Same shape as the complex melee destructor: run the destructor, free
/// through the global manager when the argument's low bit is set, return
/// `this`. Thiscall.
export!(thiscall, rw_00ccf870(this: u32, flags: u32) -> u32 {
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
