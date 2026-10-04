// original: 0x00bdf840 audio_dtor_refcount_slot
/// Destroy the audio node holding a reference-counted table slot index.
/// Stamps vtable 0xEB934C. Unless the +0x14 index is -1, drops one
/// reference on that element of the shared 0x6c-stride table; when the
/// count reaches zero and the element's +0x64 flag is set, the flag is
/// cleared and the element is shut down (id 1, thiscall/0). Forwards to
/// the shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf840(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB934C;
        const TABLE: u32 = 0x167F780;
        const STRIDE: u32 = 0x6C;
        *(this as *mut u32) = relocated(VTABLE);
        let idx = *((this.add(0x14)) as *const u32);
        if idx != 0xFFFF_FFFF {
            let elem = relocated(TABLE).wrapping_add(idx.wrapping_mul(STRIDE));
            let count = (elem.wrapping_add(0x68)) as *mut u32;
            let left = (*count).wrapping_sub(1);
            *count = left;
            if left == 0 && *((elem.wrapping_add(0x64)) as *const u8) != 0 {
                *((elem.wrapping_add(0x64)) as *mut u8) = 0;
                callee_thiscall!(1, u32, elem);
            }
        }
        callee_thiscall!(9, u32, this as u32)
    }
});
