// original: 0x00694e40 filter_set_params
/// Store two filter parameters, refresh the derived key, return the key.
///
/// Writes the two incoming floats over the object's parameter slots, calls
/// the key-derivation helper with no arguments, stores its answer in the
/// key slot and returns it.
export!(thiscall, rs80_694e40(this: *mut u8, pa: u32, pb: u32) -> u32 {
    unsafe {
        *((this).add(0x0C) as *mut u32) = pa;
        *((this).add(0x10) as *mut u32) = pb;
        let key: u32 = callee_thiscall!(1, u32, this as u32);
        *((this).add(8) as *mut u32) = key;
        key
    }
});
