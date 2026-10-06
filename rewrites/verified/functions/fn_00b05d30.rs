// original: 0x00b05d30 CCamGame::vf0
/// Deleting destructor dispatch.
///
/// thiscall `(this, flags)`: runs the real destructor (helper 1,
/// thiscall, no stack args); when bit 0 of `flags` is set it also calls
/// the allocator's free (helper 2, thiscall `(manager, this)` with the
/// manager object read from a global). Returns `this`.
export!(thiscall, rw_00b05d30(this: u32, flags: u32) -> u32 {
    const MANAGER: u32 = 0x012F_B1A0;
    let _: u32 = callee_thiscall!(1, u32, this);
    if flags & 1 != 0 {
        let mgr = unsafe { *global::<u32>(MANAGER) };
        let _: u32 = callee_thiscall!(2, u32, mgr, this);
    }
    this
});
