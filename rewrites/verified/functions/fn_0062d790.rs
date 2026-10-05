// original: 0x0062D790 procedural_texture_skyhat_init (proposed)

/// Initialise a sky-hat procedural-texture object in place.
///
/// Zeroes the flag words, small fields and pointer slots of the object at
/// `this`, stamps the class vtable pointer at `+0x00`, and writes the default
/// dimensions, version words and option flags (see the named constants).
/// Reads nothing. Returns `this` (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0062d790(this: u32) -> u32 {
    unsafe {
        const VTABLE_SKYHAT: u32 = 0xFE2478;
        const DEFAULT_DIM: u32 = 0x20;
        const DEFAULT_VERSION: u16 = 0x0101;
        const DEFAULT_ONE: u32 = 1;
        const DEFAULT_COUNT: u32 = 0x100;
        const DEFAULT_MODE: u32 = 2;
        unsafe fn w32(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        w32(this, 0x04, 0);
        w32(this, 0x08, 0);
        w32(this, 0x0c, 0);
        w32(this, 0x10, 0);
        w32(this, 0x14, 0);
        w32(this, 0x18, 0);
        w32(this, 0x1c, 0);
        w32(this, 0x20, 0);
        ((this + 0x24) as *mut u8).write(0);
        w32(this, 0x28, 0);
        w32(this, 0x2c, 0);
        ((this + 0x30) as *mut u8).write(0);
        w32(this, 0x44, 0);
        w32(this, 0x48, 0);
        w32(this, 0x00, lf_checker_rt::relocated(VTABLE_SKYHAT));
        w32(this, 0x4c, 0);
        w32(this, 0x50, 0);
        ((this + 0x54) as *mut u16).write_unaligned(0);
        ((this + 0x68) as *mut u8).write(0);
        w32(this, 0x58, DEFAULT_DIM);
        w32(this, 0x5c, DEFAULT_DIM);
        ((this + 0x60) as *mut u16).write_unaligned(DEFAULT_VERSION);
        w32(this, 0x64, DEFAULT_ONE);
        w32(this, 0x6c, DEFAULT_COUNT);
        w32(this, 0x70, DEFAULT_MODE);
        this
    }
});
