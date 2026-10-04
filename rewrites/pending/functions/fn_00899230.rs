// original: 0x00899230 audio_ptrkey_insert_or_update
/// Inserts or updates a key in a 24-entry slot table.
///
/// Stores the key and value into the matching slot, or into the first empty
/// slot when the key is absent (a match only rewrites the value half).
/// Returns the slot index used, or 24 when every slot holds another key.
export!(thiscall, rw_00899230(this: u32, key: u32, value: f32) -> u32 {
    unsafe {
        for i in 0..24u32 {
            let slot = this.wrapping_add(i.wrapping_mul(8));
            let v = core::ptr::read_unaligned(slot as *const u32);
            if v == key || v == 0 {
                core::ptr::write_unaligned(slot as *mut u32, key);
                core::ptr::write_unaligned(slot.wrapping_add(4) as *mut f32, value);
                return i;
            }
        }
        24
    }
});
