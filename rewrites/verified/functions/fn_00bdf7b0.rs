// original: 0x00bdf7b0 audio_dtor_stream_and_handle
/// Destroy the audio node holding a stream pair and a handle.
/// Stamps vtable 0xEB8EE4. When the +0x20 word is non-null and the +0x1c
/// word is non-null, the pair is closed (id 1, stdcall/2) and the returned
/// stage is shut down (id 2, thiscall/0). When the +0x18 word is non-null
/// its slot is released through the shared release helper (id 3). Forwards
/// to the shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf7b0(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8EE4;
        *(this as *mut u32) = relocated(VTABLE);
        let hi = *((this.add(0x20)) as *const u32);
        if hi != 0 {
            let lo = *((this.add(0x1C)) as *const u32);
            if lo != 0 {
                // Stack order is push lo, push hi: hi lands in arg0.
                let stage = callee_stdcall!(1, u32, hi, lo);
                callee_thiscall!(2, u32, stage);
            }
        }
        if *((this.add(0x18)) as *const u32) != 0 {
            callee_stdcall!(3, u32, this.add(0x18) as u32);
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
