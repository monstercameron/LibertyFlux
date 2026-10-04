// original: 0x008a37d0 sound_ctor_fragment
/// Constructor fragment that stamps the sound vtable.
///
/// Calls the base constructor (thiscall/0, stubbed), writes the class vtable
/// pointer, and returns the object pointer.
export!(thiscall, rw_008a37d0(this: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this);
        *((this as *mut u8) as *mut u32) = relocated(0xe7accc);
        this
    }
});
