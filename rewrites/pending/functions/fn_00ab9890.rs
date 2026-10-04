// original: 0x00ab9890 slot_search_copy_pair

/// Search two stride-64 records by key, copy two header fields on a hit.
///
/// Compares the key against the header word of each of the two records at
/// `this`. On a hit copies the dwords at offsets 0x0c and 0x10 into the two
/// output slots. Returns the second output pointer with its low byte set on
/// a hit (the original reloads EAX with that pointer and sets AL), else 0.
export!(thiscall, rs64_ab9890(this: *const u8, key: u32, out0: *mut u32, out1: *mut u32) -> u32 {
    unsafe {
        const COUNT: u32 = 2;
        const STRIDE: u32 = 64;
        let base = this as u32;
        let mut i = 0u32;
        while i < COUNT {
            let slot = base.wrapping_add(i.wrapping_mul(STRIDE));
            if *(slot as *const u32) == key {
                *out0 = *((slot.wrapping_add(0x0c)) as *const u32);
                *out1 = *((slot.wrapping_add(0x10)) as *const u32);
                return (out1 as u32 & 0xFFFF_FF00) | 1;
            }
            i += 1;
        }
        0
    }
});
