// original: 0x00899260 audio_named_key_insert
/// Resolves a name to a key, then inserts or updates it in the slot table.
///
/// Hashes the name operand through the key resolver and stores the resulting
/// key with the value into the matching or first empty slot. Returns the key.
export!(thiscall, rw_00899260(this: u32, name: u32, value: f32) -> u32 {
    unsafe {
        let key = callee_cdecl!(1, u32, name, 0);
        for i in 0..24u32 {
            let slot = this.wrapping_add(i.wrapping_mul(8));
            let v = core::ptr::read_unaligned(slot as *const u32);
            if v == key || v == 0 {
                core::ptr::write_unaligned(slot as *mut u32, key);
                core::ptr::write_unaligned(slot.wrapping_add(4) as *mut f32, value);
                break;
            }
        }
        key
    }
});
