// original: 0x008a5fd0 rage::audMathOperationSound::audMathOperationSound
/// Construct an `audMathOperationSound` (math audio node) in place.
///
/// Runs the shared base constructor, installs this class's function table
/// and sets the operation byte at +0xB5 to 0xFF (none selected). Returns
/// the object pointer.
export!(thiscall, rw_008a5fd0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(0x00E7B974);
        *this.add(0xB5) = 0xFF;
        this as u32
    }
});

