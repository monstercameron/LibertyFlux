// original: 0x0088F7C0 rage::audSound::vf5
// 0088F7C0 rage::audSound::vf5: reset the sound, free it through the pool
// when the delete flag is set, and return the object.
export!(thiscall, rw_0088f7c0(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        if flags & 1 != 0 && !this.is_null() {
            callee_thiscall!(
                2,
                u32,
                relocated(0x115D8A0),
                this as u32,
                *this.add(0x40) as u32
            );
        }
        this as u32
    }
});
