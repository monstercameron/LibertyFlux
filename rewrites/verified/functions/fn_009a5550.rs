// original: 0x009a5550 script_audio_bind_handle_and_name
/// Original 0x009a5550 (unnamed): bind a handle and copy its name.
///
/// Stores `h` into the slot for `idx`, runs the bind helper on the slot
/// (clearing the bound flag) unless `h` is null (setting it instead), then
/// copies the NUL-terminated name at `name` into the slot's name field.
/// Returns the end of the source string.
export!(thiscall, rw_009a5550(this_: u32, idx: u32, h: u32, name: u32) -> u32 {
    unsafe {
        let slot = this_.wrapping_add(idx.wrapping_add(0xb85).wrapping_mul(4));
        (slot as *mut u32).write(h);
        let flag = this_.wrapping_add(idx).wrapping_add(0x32c1);
        if h != 0 {
            callee_thiscall!(1, u32, h, slot);
            (flag as *mut u8).write(0);
        } else {
            (flag as *mut u8).write(1);
        }
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
        s
    }
});
