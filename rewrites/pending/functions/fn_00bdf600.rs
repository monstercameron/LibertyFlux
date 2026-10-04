// original: 0x00bdf600 audio_dtor_release_pair
/// Destroy the audio node holding two referenced slots.
/// Stamps vtable 0xEB8B2C, then for each of the +0x14/+0x18 slots: when
/// non-null, releases it through slot 0 of its own vtable (planted id 1,
/// stdcall/1 with argument 1) and clears the slot. Forwards to the shared
/// base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf600(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8B2C;
        *(this as *mut u32) = relocated(VTABLE);
        for off in [0x14usize, 0x18usize] {
            let p = *((this.add(off)) as *const u32);
            if p != 0 {
                let vtable = *(p as *const u32);
                let target = *(vtable as *const u32);
                let release: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(target as usize);
                release(1);
                *((this.add(off)) as *mut u32) = 0;
            }
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
