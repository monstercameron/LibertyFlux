// original: 0x00c2c610 audEntityRadioEmitter::vf13
/// Handle lookup check.
///
/// Returns false for a null handle, otherwise queries the registry with
/// two zero keys and returns whether the answer equals the handle.
export!(thiscall, rw_00c2c610(this: *const u8) -> u32 {
    unsafe {
        let handle = *(this.add(4) as *const u32);
        if handle == 0 {
            return 0;
        }
        let answer: u32 = callee_cdecl!(1, u32, 0, 0);
        (answer == handle) as u32
    }
});
