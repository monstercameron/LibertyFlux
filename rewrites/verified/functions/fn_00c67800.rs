// original: 0x00c67800 CCutsceneObject::vf54
/// Clears the byte at 0xbc, then forwards (0, -2) to slot 2 of the helper
/// at [this+0x80], addressed at helper+0x80.
export!(thiscall, rw_c67800(this: u32) -> u32 {
    unsafe {
        ((this as *mut u8).byte_add(0xbc)).write(0);
        let vtable = (this as *const u32).byte_add(0x80).read();
        let slot = (vtable as *const u32).byte_add(8).read();
        let target: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        target(this.wrapping_add(0x80), 0, 0xFFFF_FFFEu32)
    }
});
