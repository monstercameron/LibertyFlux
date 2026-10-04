// original: 0x0089ff60 rage::audRetriggeredOverlappedSound::vf5
/// Deleting destructor of `rage::audRetriggeredOverlappedSound` (vf5).
///
/// Runs the object destructor (thiscall/0, stubbed), then frees the object
/// through the sound pool when the low flag bit is set and the pointer is
/// non-null. Returns the object pointer.
export!(thiscall, rw_0089ff60(this: u32, flags: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        if flags & 1 != 0 && this != 0 {
            let bank = *((this as *const u8).add(0x40));
            callee_thiscall!(2, u32, relocated(SOUND_POOL), this, bank as u32);
        }
        this
    }
});
