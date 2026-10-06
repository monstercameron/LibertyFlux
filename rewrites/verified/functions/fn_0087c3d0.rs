// original: 0x0087c3d0 rage::crmtUpdater::vf3
/// Attach an updater to a subject: notify the subject, then link this.
///
/// Notifies slot 0x0c of the subject's table with (subject, this), then runs
/// intercepted direct callee 2 (thiscall/1) with (this, subject). Computes
/// no return value.
///
/// Original: thiscall/1, one indirect plus one direct call, no floats.
export!(thiscall, rw_0087c3d0(this: u32, subject: u32) -> u32 {
    /// Notify slot in the subject's table.
    const NOTIFY_SLOT: u32 = 0x0C;
    unsafe {
        let vt = (subject as *const u32).read_unaligned();
        let tgt = ((vt + NOTIFY_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        f(subject, this);
        callee_thiscall!(2, u32, this, subject);
        0
    }
});
