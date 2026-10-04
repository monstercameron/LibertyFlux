// original: 0x009e12d0 audio_tracker_unregister_if_flagged
/// When the object is registered (flag byte set), unregister it
/// through the list helper and clear the flag. No defined return (EAX keeps
/// its entry value on the skip path). (cdecl/1)
export!(cdecl, rw_009e12d0(obj: u32) -> u32 {
    unsafe {
        if *((obj.wrapping_add(0x44)) as *const u8) != 0 {
            callee_thiscall!(1, u32, relocated(0x12B41B4), obj);
            *((obj.wrapping_add(0x44)) as *mut u8) = 0;
        }
        0
    }
});
