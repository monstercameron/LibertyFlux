// original: 0x006262d0 rage::snHandleJoinRequestTask::snHandleJoinRequestTask
// Reset the held task object through its teardown step and retag it.
//
// Does nothing when the holder is empty. Otherwise runs the teardown call
// on the held object, clears its state words, sets the initialised flag,
// retags it and clears its tail words. Returns the held object.
export!(thiscall, rw_006262d0(holder: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x00fe33b0;
        let target: u32 = *(holder as *const u32);
        if target == 0 {
            return 0;
        }
        callee_thiscall!(1, u32, target);
        *((target.wrapping_add(0x60)) as *mut u32) = 0;
        *((target.wrapping_add(0x64)) as *mut u32) = 0;
        *((target.wrapping_add(0x68)) as *mut u32) = 0;
        *((target.wrapping_add(0x6c)) as *mut u8) |= 1;
        *((target.wrapping_add(0x6d)) as *mut u8) = 0;
        *(target as *mut u32) = relocated(TAG);
        callee_thiscall!(2, u32, target.wrapping_add(0x90));
        target
    }
});
