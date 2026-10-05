// original: 0x00937B20 stream_obj_zero_init_a (proposed)

/// Zero the header and flag fields of a streaming object, returning the object.
///
/// `this` points to the object. Writes dword 0 at `+0x00`, `+0x04`, `+0x08`,
/// `+0x0C`, byte 0 at `+0x10` and `+0x10F`, byte 0 at `+0x30D` and dword 0 at
/// `+0x310`; all other bytes are untouched. Returns `this` (thiscall).
lf_checker_rt::export!(thiscall, rw_00937b20(this: u32) -> u32 {
    unsafe {
        const HDR_WORDS: u32 = 4;
        const FLAG_A: u32 = 0x10;
        const FLAG_B: u32 = 0x10F;
        const FLAG_C: u32 = 0x30D;
        const TAIL: u32 = 0x310;
        for i in 0..HDR_WORDS {
            ((this + i * 4) as *mut u32).write_unaligned(0);
        }
        ((this + FLAG_A) as *mut u8).write(0);
        ((this + FLAG_B) as *mut u8).write(0);
        ((this + FLAG_C) as *mut u8).write(0);
        ((this + TAIL) as *mut u32).write_unaligned(0);
        this
    }
});
