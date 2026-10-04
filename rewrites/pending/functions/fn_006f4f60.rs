// original: 0x006f4f60 seeded_object_init
/// One-time seeds the shared generator, then initializes the object.
///
/// On the first call (flag clear) mixes the platform helper's answer with a
/// rotate-xor-multiply hash, publishes the 64-bit seed to the shared slots,
/// and sets the flag. Every call zeroes slots 0 to 2, tags the kind byte at
/// offset 5 with 2, and returns the object pointer.
export!(thiscall, rw_006f4f60(this: u32) -> u32 {
    unsafe {
        if *global::<u8>(0x18B8914) == 0 {
            let s = callee_cdecl!(1, u32, 0);
            let r = callee_cdecl!(2, u32,);
            let mixed = r.wrapping_add(s);
            let t = mixed.wrapping_add((mixed == 0) as u32);
            let fold = mixed.rotate_left(16) ^ mixed;
            let prod = (t as u64).wrapping_mul(0x5CDCFAA7);
            let sum = (prod as u32) as u64 + fold as u64;
            *global::<u8>(0x18B8914) = 1;
            *global::<u32>(0x1110458) = sum as u32;
            *global::<u32>(0x111045C) =
                ((prod >> 32) as u32).wrapping_add((sum >> 32) as u32);
        }
        *(this as *mut u64) = 0;
        *((this + 8) as *mut u32) = 0;
        *((this + 5) as *mut u8) = 2;
        this
    }
});
