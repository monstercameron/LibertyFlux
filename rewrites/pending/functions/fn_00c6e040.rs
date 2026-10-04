// original: 0x00c6e040 anim_init_vec3_pair
/// Initialise an animation node: id word to -1, then two copies of a global
/// triple of floats (re-read from the globals for the second copy, as the
/// original does). Returns `this`.
export!(thiscall, rw_00c6e040(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = 0xFFFF_FFFF;
        *(this as *mut u32).add(4) = *global::<u32>(0x01B4_B2A0);
        *(this as *mut u32).add(5) = *global::<u32>(0x01B4_B2A4);
        *(this as *mut u32).add(6) = *global::<u32>(0x01B4_B2A8);
        *(this as *mut u32).add(8) = *global::<u32>(0x01B4_B2A0);
        *(this as *mut u32).add(9) = *global::<u32>(0x01B4_B2A4);
        *(this as *mut u32).add(10) = *global::<u32>(0x01B4_B2A8);
        this
    }
});
