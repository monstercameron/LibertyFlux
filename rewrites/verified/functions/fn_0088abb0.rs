// original: 0x0088abb0 audio_store_triplet
/// Stores three bytes into this object's indexed triplet slots.
///
/// The index lives at byte `0xC0`; slot `i` holds three bytes at offsets
/// `3*i`, `3*i+1`, `3*i+2`. The index is re-read before each store and
/// incremented (wrapping) afterwards. Returns the third byte.
export!(thiscall, rw_0088abb0(this_ptr: *mut u8, a0: u32, a1: u32, a2: u32) -> u8 {
    unsafe {
        let idx0 = *this_ptr.add(0xC0);
        *this_ptr.add((idx0 as u32).wrapping_mul(3) as usize) = a0 as u8;
        let idx1 = *this_ptr.add(0xC0);
        *this_ptr.add((idx1 as u32).wrapping_mul(3).wrapping_add(1) as usize) = a1 as u8;
        let idx2 = *this_ptr.add(0xC0);
        *this_ptr.add((idx2 as u32).wrapping_mul(3).wrapping_add(2) as usize) = a2 as u8;
        let n = *this_ptr.add(0xC0);
        *this_ptr.add(0xC0) = n.wrapping_add(1);
        a2 as u8
    }
});
