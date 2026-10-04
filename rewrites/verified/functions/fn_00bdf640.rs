// original: 0x00bdf640 audio_dtor_release_one
/// Destroy the audio node holding one referenced slot at +0x40.
/// Stamps vtable 0xEB92EC; when the slot is non-null releases it through
/// slot 0 of its own vtable (planted id 1, stdcall/1 with argument 1).
/// Forwards to the shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf640(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB92EC;
        *(this as *mut u32) = relocated(VTABLE);
        let p = *((this.add(0x40)) as *const u32);
        if p != 0 {
            let vtable = *(p as *const u32);
            let target = *(vtable as *const u32);
            let release: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            release(1);
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
