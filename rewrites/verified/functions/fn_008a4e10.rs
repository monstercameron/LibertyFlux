// original: 0x008a4e10 rage::audIfSound::audIfSound
/// Construct an `audIfSound` (conditional audio node) in place.
///
/// Runs the shared base constructor, installs this class's function table,
/// then sets the mode word at +0xC0 to 2 and clears the flag byte at +0xC4.
/// Returns the object pointer.
export!(thiscall, rw_008a4e10(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(0x00E7B6A4);
        *(this.add(0xC0) as *mut u32) = 2;
        *this.add(0xC4) = 0;
        this as u32
    }
});

