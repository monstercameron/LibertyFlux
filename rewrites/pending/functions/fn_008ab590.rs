// original: 0x008AB590 audio_channel_reset
/// Reset a channel: re-initialise the sub-object at offset 0x20 and clear
/// the trailing state word. Returns the object.
export!(thiscall, rw_008AB590(obj: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, (obj as u32).wrapping_add(0x20));
        *(obj.add(0x48) as *mut u32) = 0;
        obj as u32
    }
});
