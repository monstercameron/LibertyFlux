// original: 0x008a09b0 rage::audCrossfadeSound::audCrossfadeSound
/// Constructor for the crossfade sound object.
///
/// Runs the base constructor, installs this class's virtual table, marks the
/// crossfade byte at offset `0xB0` as unset (`0xFF`), and returns `this`.
export!(thiscall, rw_008a09b0(this: *mut u8) -> u32 {
    const VTABLE: u32 = 0xe7a730;
    const XFADE_OFF: usize = 0xb0;
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(VTABLE);
        *this.add(XFADE_OFF) = 0xff;
        this as u32
    }
});
