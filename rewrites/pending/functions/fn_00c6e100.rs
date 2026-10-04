// original: 0x00c6e100 anim_init_vec4_pair
/// Initialise an animation node: id word to -1, then two copies of a global
/// quadruple of floats. No usable return value.
export!(thiscall, rw_00c6e100(this: u32) -> () {
    unsafe {
        *(this as *mut u32) = 0xFFFF_FFFF;
        *(this as *mut u32).add(4) = *global::<u32>(0x01B4_B2A0);
        *(this as *mut u32).add(5) = *global::<u32>(0x01B4_B2A4);
        *(this as *mut u32).add(6) = *global::<u32>(0x01B4_B2A8);
        *(this as *mut u32).add(7) = *global::<u32>(0x01B4_B2AC);
        *(this as *mut u32).add(8) = *global::<u32>(0x01B4_B2A0);
        *(this as *mut u32).add(9) = *global::<u32>(0x01B4_B2A4);
        *(this as *mut u32).add(10) = *global::<u32>(0x01B4_B2A8);
        *(this as *mut u32).add(11) = *global::<u32>(0x01B4_B2AC);
    }
});
