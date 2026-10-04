// original: 0x009a55b0 script_audio_bind_slot_name
/// Original 0x009a55b0 (unnamed): bind a name to a script-audio slot.
///
/// Clears the handle word for slot `idx`, copies the NUL-terminated name at
/// `name` into the slot's 64-byte name field, and marks the slot bound.
/// Returns the end of the source string.
export!(thiscall, rw_009a55b0(this_: u32, idx: u32, name: u32) -> u32 {
    unsafe {
        let slot = this_.wrapping_add(idx.wrapping_mul(4)).wrapping_add(0x2e14);
        (slot as *mut u32).write(0);
        let mut s = name;
        let mut d = this_
            .wrapping_add(idx.wrapping_mul(64))
            .wrapping_add(0x3078);
        loop {
            let b = (s as *const u8).read();
            (d as *mut u8).write(b);
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        let flag = this_.wrapping_add(idx).wrapping_add(0x32c1);
        (flag as *mut u8).write(1);
        s
    }
});
