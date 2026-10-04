// original: 0x009b77a0 rw_009b77a0
/// Reset the camera object: clear two globals, stamp the default field
/// values, run the element initialiser, report success in AL (upper bytes
/// carry through from the initialiser's answer, as in the original).
export!(thiscall, rw_009b77a0(this_: u32) -> u32 {
    unsafe {
        const FLAG_G: u32 = 0x128E3FC;
        const MODE_G: u32 = 0x1039298;
        const TIMEOUT: u32 = 0x3E8;
        const FAR_PLANE: u32 = 0xC7C3_4F80;
        *global::<u32>(FLAG_G) = 0;
        *global::<u32>(MODE_G) = 0xFFFF_FFFF;
        core::ptr::write_unaligned((this_.wrapping_add(0x531)) as *mut u32, 0);
        *((this_.wrapping_add(0x530)) as *mut u8) = 0;
        *((this_.wrapping_add(0x528)) as *mut u32) = TIMEOUT;
        *((this_.wrapping_add(0x538)) as *mut u32) = FAR_PLANE;
        *((this_.wrapping_add(0x53C)) as *mut u32) = FAR_PLANE;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ans = init(this_);
        (ans & 0xFFFF_FF00) | 1
    }
});
