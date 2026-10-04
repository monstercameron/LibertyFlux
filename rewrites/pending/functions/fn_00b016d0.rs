// original: 0x00b016d0 CViewport::vf1
/// Refresh the viewport: bump the revision, re-initialise the child list and
/// re-register with the manager, keeping the returned handle.
export!(thiscall, rw_00b016d0(this: u32) -> u32 {
    unsafe {
        let revision = global::<u32>(0x1601090);
        *revision = (*revision).wrapping_add(1);
        callee_thiscall!(1, u32, this.wrapping_add(0x400));
        let handle = callee_thiscall!(2, u32, relocated(0x161547c), this, 1, 0);
        *((this + 0x540) as *mut u32) = handle;
        *((this + 0x538) as *mut u32) = 0;
        0
    }
});
