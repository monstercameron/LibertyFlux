// original: 0x008a58a0 rage::audForLoopSound::audForLoopSound
/// Construct an `audForLoopSound` (looping audio node) in place.
///
/// Runs the shared base constructor, installs this class's function table,
/// marks the two loop counters at +0xB0/+0xB4 as unset (-1), sets the two
/// gain words at +0xB8/+0xBC to 1.0, and zeroes the mode dword at +0xC0 and
/// the flag word at +0xC4. Returns the object pointer.
export!(thiscall, rw_008a58a0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(0x00E7B80C);
        *(this.add(0xB0) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0xB4) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0xB8) as *mut f32) = 1.0;
        *(this.add(0xBC) as *mut f32) = 1.0;
        *(this.add(0xC0) as *mut u32) = 0;
        *(this.add(0xC4) as *mut u16) = 0;
        this as u32
    }
});

