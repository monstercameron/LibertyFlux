// original: 0x008a3cc0 rage::audSwitchSound::vf5
/// Deleting destructor of `audSwitchSound`.
///
/// Original 0x008A3CC0 (`thiscall(this, flag)`): runs the scalar
/// destructor body, then frees the object through the audio heap when
/// `flag & 1` (and `this` is non-null). Returns `this`.
export!(thiscall, rw_008a3cc0(this: u32, flag: u32) -> u32 {    callee_thiscall!(1, u32, this);
    if flag & 1 != 0 && this != 0 {
        let bank = unsafe { ((this + 0x40) as *const u8).read_unaligned() } as u32;
        callee_thiscall!(2, u32, relocated(0x115d8a0), this, bank);
    }
    this
});
