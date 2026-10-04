// original: 0x00bfa7d0 composite_init_marker47
//! rs20f1 @0xBFA7D0: composite init (thiscall/8). Runs the 3-arg sub-init,
//! stamps the 0x47 marker, runs the vec3 field init, then packs two flag bits,
//! a word and a float into the tail of the object. EAX keeps the second
//! callee's high word over the stored low word.
export!(thiscall, rw_rs20f1(
    this: *mut u8,
    a1: u32, a2: u32, a3: u32, a4: u32,
    a5: u32, a6: u32, a7: u32, a8: u32,
) -> u32 {
    unsafe {
        let sub_init: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        sub_init(this as u32, a1, a2, a3);
        *this = 0x47;
        *this.add(2) = 0x0f;
        let vec_init: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let r2 = vec_init(this as u32, a4);
        *this.add(0x1f) = (((a7 & 1) << 1) | (a6 & 1)) as u8;
        *(this.add(0x20) as *mut u16) = a5 as u16;
        *(this.add(0x24) as *mut u32) = a8;
        (r2 & 0xFFFF0000) | (a5 & 0xFFFF)
    }
});
