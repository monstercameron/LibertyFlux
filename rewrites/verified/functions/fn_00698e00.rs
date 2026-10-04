// original: 0x00698e00 unknown (animation channel copy constructor)
/// Copy constructor for a quantized animation channel: stamps the vtable,
/// copies the two flag bytes and the count word, delegates the three
/// sub-objects (+0x08, +0x14, +0x20) to their own copiers, then copies the
/// three trailing parameter words. Returns this.
lf_k2_rt::export!(thiscall, rw_00698e00(this: *mut u8, src: *const u8) -> u32 {
    unsafe {
        // Intermediate vtable, overwritten below once the header is copied.
        *(this as *mut u32) = lf_k2_rt::relocated(0x00FE3A74);
        *this.add(4) = *src.add(4);
        *this.add(5) = *src.add(5);
        *((this.add(6)) as *mut u16) = *((src.add(6)) as *const u16);
        *(this as *mut u32) = lf_k2_rt::relocated(0x00FE3B34);
        lf_k2_rt::callee_thiscall!(
            1,
            u32,
            (this as u32).wrapping_add(8),
            (src as u32).wrapping_add(8)
        );
        lf_k2_rt::callee_thiscall!(
            1,
            u32,
            (this as u32).wrapping_add(0x14),
            (src as u32).wrapping_add(0x14)
        );
        lf_k2_rt::callee_thiscall!(
            2,
            u32,
            (this as u32).wrapping_add(0x20),
            (src as u32).wrapping_add(0x20)
        );
        *((this.add(0x2c)) as *mut u32) = *((src.add(0x2c)) as *const u32);
        *((this.add(0x30)) as *mut u32) = *((src.add(0x30)) as *const u32);
        *((this.add(0x34)) as *mut u32) = *((src.add(0x34)) as *const u32);
        this as u32
    }
});
