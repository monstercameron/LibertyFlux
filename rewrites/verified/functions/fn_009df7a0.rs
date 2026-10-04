// original: 0x009df7a0 CPortalTracker::vf0
// fn_009df7a0: CPortalTracker::vf0, scalar deleting destructor (thiscall/1).
//
// Runs the reset-and-detach step, frees the object through the game's
// allocator when the low flag bit is set, and returns `this`.
export!(thiscall, rw_009df7a0(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        let cleanup: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        cleanup(this as u32);
        if flags & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
