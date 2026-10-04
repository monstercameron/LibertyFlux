// original: 0x00b09da0 CCamGame::vf1
/// Restart the game camera through its worker object.
///
/// Runs the camera's own teardown step, builds a fresh worker for the
/// owner, releases the worker, then clears the active marker and the
/// pending words. Always returns 1.
export!(thiscall, rw_00b09da0(this_ptr: u32) -> u32 {
    unsafe {
        const TEARDOWN_SLOT: usize = 0x18;
        const RELEASE_SLOT: usize = 0x04;
        let table = *(this_ptr as *const u32) as usize;
        let step_at = *((table + TEARDOWN_SLOT) as *const u32) as usize;
        let step: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(step_at);
        step(this_ptr);
        let owner = *((this_ptr as usize + 0x114) as *const u32);
        let worker = callee_thiscall!(2, u32, owner, 1u32, 0u32, this_ptr);
        callee_thiscall!(3, u32, worker);
        let wtable = *(worker as *const u32) as usize;
        let rel_at = *((wtable + RELEASE_SLOT) as *const u32) as usize;
        let release: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(rel_at);
        release(worker);
        *((this_ptr as usize + 0x1c4) as *mut u8) &= 0xbf;
        *((this_ptr as usize + 0x1c8) as *mut u32) = 0;
        *((this_ptr as usize + 0x1cc) as *mut u32) = 0;
        *((this_ptr as usize + 0x1d0) as *mut u32) = 0;
        1
    }
});
