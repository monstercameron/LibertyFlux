// original: 0x00bdf760 audio_dtor_gain_stage
/// Destroy the audio node holding a gain stage at +0x1c.
/// Stamps vtable 0xEB8DD4. When the stage is non-null it is silenced with
/// gain -1000.0f (id 1) then detached (id 2), and the slot is cleared.
/// Forwards to the shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf760(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8DD4;
        const SILENCE_GAIN_BITS: u32 = 0xC47A_0000;
        *(this as *mut u32) = relocated(VTABLE);
        if *((this.add(0x1C)) as *const u32) != 0 {
            callee_stdcall!(1, u32, SILENCE_GAIN_BITS);
            callee_stdcall!(2, u32, this as u32);
            *((this.add(0x1C)) as *mut u32) = 0;
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
