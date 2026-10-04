// original: 0x008916a0 rage::audSound::vf3
/// Virtual slot 2 helper: hashes the key, then forwards.
///
/// Calls the hash callee with the key and zero, then invokes the object's
/// virtual slot 2 with this object, the hash answer and the float bits.
/// Returns the slot's answer.
export!(thiscall, rw_008916a0(this: *mut u8, key: u32, value_bits: u32) -> u32 {
    unsafe {
        let hashed: u32 = callee_cdecl!(1, u32, key, 0);
        let vtable = *(this as *const u32);
        let target = *((vtable.wrapping_add(8)) as *const u32);
        let store: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        store(this as u32, hashed, value_bits)
    }
});
