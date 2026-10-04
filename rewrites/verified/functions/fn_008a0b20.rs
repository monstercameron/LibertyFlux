// original: 0x008a0b20 rage::audCrossfadeSound::vf5
/// Deleting destructor of `rage::audCrossfadeSound` (vf5).
///
/// Same shape as [`rw_0089ff60`]: destructor body then conditional pool free.
export!(thiscall, rw_008a0b20(this: u32, flags: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        if flags & 1 != 0 && this != 0 {
            let bank = *((this as *const u8).add(0x40));
            callee_thiscall!(2, u32, relocated(SOUND_POOL), this, bank as u32);
        }
        this
    }
});
