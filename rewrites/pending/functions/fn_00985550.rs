// original: 0x00985550 audio_emitter_init
/// Initialize an audio emitter object in place.
///
/// Stores the vtable reference (relocated), state words, float constants
/// (1.0, 20.0), id words and zeroed regions at their fixed offsets, and
/// returns `this`. Several stores are intentionally unaligned.
export!(thiscall, rw_00985550(this: u32) -> u32 {
    unsafe {
        use core::ptr::write_unaligned as wu;
        let p = this;
        wu((p.wrapping_add(0x10)) as *mut u32, relocated(0xE8E284));
        wu((p.wrapping_add(0x4c)) as *mut u32, 0xFFFFFFFF);
        wu((p.wrapping_add(0x48)) as *mut u32, 0xFFFFFFFF);
        wu((p.wrapping_add(0x50)) as *mut u8, 0);
        wu((p.wrapping_add(0x34)) as *mut u32, 0x5D5C);
        wu((p.wrapping_add(0x38)) as *mut u32, 0);
        wu((p.wrapping_add(0x3c)) as *mut u32, 0x3F800000);
        wu((p.wrapping_add(0x60)) as *mut u8, 0xFF);
        wu((p.wrapping_add(0x61)) as *mut u32, 0xFFFFFFFF);
        wu((p.wrapping_add(0x65)) as *mut u32, 0xAAAAAAAA);
        wu((p.wrapping_add(0x6a)) as *mut u32, 0xE38FCF16);
        wu((p.wrapping_add(0x6e)) as *mut u32, 0x1D632BA1);
        wu((p.wrapping_add(0x7e)) as *mut u32, 0);
        wu((p.wrapping_add(0x82)) as *mut u32, 0x41A00000);
        wu((p.wrapping_add(0x86)) as *mut u32, 0);
        wu((p.wrapping_add(0x8a)) as *mut u32, 0x5D5C);
        wu((p.wrapping_add(0x90)) as *mut u32, 0);
        wu((p.wrapping_add(0x94)) as *mut u32, 0);
        wu((p.wrapping_add(0x8e)) as *mut u16, 0x64);
        wu((p.wrapping_add(0xa4)) as *mut u32, 0);
        wu((p.wrapping_add(0xc5)) as *mut u16, 0);
        wu((p.wrapping_add(0x98)) as *mut u32, 0);
        wu((p.wrapping_add(0x9c)) as *mut u32, 0);
        wu((p.wrapping_add(0xa0)) as *mut u32, 0);
        wu((p.wrapping_add(0xc1)) as *mut u16, 0);
        wu((p.wrapping_add(0xac)) as *mut u32, 0);
        wu((p.wrapping_add(0xc0)) as *mut u8, 0);
        wu((p.wrapping_add(0xb8)) as *mut u32, 0);
        wu((p.wrapping_add(0xbc)) as *mut u32, 0);
        this
    }
});
