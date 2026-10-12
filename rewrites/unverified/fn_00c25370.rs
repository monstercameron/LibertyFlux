// original: 0x00c25370 cam_interp_style_config_c
/// Initializes the camera interpolation style fields on `this`. The two callback slots receive the relocated shared callback, and the remaining five dwords receive this style's fixed mode values. The two stack arguments are ignored; the function returns zero.
lf_checker_rt::export!(thiscall, rw_00c25370(this: u32, _unused0: u32, _unused1: u32) -> u32 {
    unsafe {
        const CALLBACK_VA: u32 = 0x00c24970;
        const CALLBACK_A: usize = 0x174;
        const CALLBACK_B: usize = 0x170;
        const STYLE_MODE: usize = 0x15c;
        const STYLE_BLEND: usize = 0x160;
        const STYLE_FLAGS: usize = 0x164;
        const STYLE_TARGET: usize = 0x168;
        const STYLE_RATE: usize = 0x16c;
        let object = this as usize as *mut u8;
        let callback = lf_checker_rt::relocated(CALLBACK_VA);
        core::ptr::write_unaligned(object.add(CALLBACK_A) as *mut u32, callback);
        core::ptr::write_unaligned(object.add(CALLBACK_B) as *mut u32, callback);
        core::ptr::write_unaligned(object.add(STYLE_MODE) as *mut u32, 1);
        core::ptr::write_unaligned(object.add(STYLE_BLEND) as *mut u32, 1);
        core::ptr::write_unaligned(object.add(STYLE_FLAGS) as *mut u32, 0);
        core::ptr::write_unaligned(object.add(STYLE_TARGET) as *mut u32, 0);
        core::ptr::write_unaligned(object.add(STYLE_RATE) as *mut u32, 0);
    }
    0
});