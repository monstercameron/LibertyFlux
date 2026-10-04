// original: 0x00bdf690 audio_dtor_handle_and_gain
/// Destroy the audio node holding a handle at +0x3c and a gain stage at +0x38.
/// Stamps vtable 0xEB94AC. When the handle is non-null it is released
/// through the shared release helper (id 1, the +0x3c slot address) and the
/// slot is cleared. When the gain stage is non-null it is detached (id 2)
/// then silenced with gain -4.0f (id 3), and the slot is cleared. Forwards
/// to the shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf690(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB94AC;
        const SILENCE_GAIN_BITS: u32 = 0xC080_0000;
        *(this as *mut u32) = relocated(VTABLE);
        if *((this.add(0x3C)) as *const u32) != 0 {
            callee_stdcall!(1, u32, this.add(0x3C) as u32);
            *((this.add(0x3C)) as *mut u32) = 0;
        }
        if *((this.add(0x38)) as *const u32) != 0 {
            callee_stdcall!(2, u32, this as u32);
            callee_stdcall!(3, u32, SILENCE_GAIN_BITS);
            *((this.add(0x38)) as *mut u32) = 0;
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
