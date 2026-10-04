// original: 0x00bdf900 audio_dtor_voice_and_handle
/// Destroy the audio node holding a voice at +0x18 and a handle at +0x14.
/// Stamps vtable 0xEB93FC. When the voice is non-null it is detached
/// (id 1) and the slot is cleared. When the handle is non-null its slot
/// is released through the shared release helper (id 2) and cleared.
/// Forwards to the shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf900(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB93FC;
        *(this as *mut u32) = relocated(VTABLE);
        if *((this.add(0x18)) as *const u32) != 0 {
            callee_stdcall!(1, u32, this as u32);
            *((this.add(0x18)) as *mut u32) = 0;
        }
        if *((this.add(0x14)) as *const u32) != 0 {
            callee_stdcall!(2, u32, this.add(0x14) as u32);
            *((this.add(0x14)) as *mut u32) = 0;
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
