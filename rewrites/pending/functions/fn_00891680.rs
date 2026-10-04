// original: 0x00891680 rage::audSound::vf2
/// Virtual slot 0 helper: stores a float through a looked-up pointer.
///
/// Calls the object's first virtual slot with the given key; when it returns
/// a non-null pointer, stores the float argument there. Returns the pointer.
export!(thiscall, rw_00891680(this: *mut u8, key: u32, value_bits: u32) -> u32 {
    unsafe {
        let vtable = *(this as *const u32);
        let lookup: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*(vtable as *const u32) as usize);
        let slot = lookup(this as u32, key);
        if slot != 0 {
            *(slot as *mut u32) = value_bits;
        }
        slot
    }
});
