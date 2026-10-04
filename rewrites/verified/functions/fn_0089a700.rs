// original: 0x0089a700 aud_obj_init_3elem
/// Initializes an audio object holding three embedded elements.
///
/// Runs the element initializer over the three consecutive slots at the
/// object's base, then stores the default tuning fields (two cleared
/// indices, two unit gains, zeroed spares, a released marker and a zero
/// flag). Returns the object pointer.
export!(thiscall, rw_0089a700(this: *mut u8) -> u32 {
    unsafe {
        let base = this as u32;
        let mut slot = 0u32;
        while slot < 3 {
            callee_thiscall!(1, u32, base + slot * 0x28);
            slot += 1;
        }
        *(this.add(0x78) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0x7C) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0x80) as *mut u32) = 0x3F80_0000;
        *(this.add(0x84) as *mut u32) = 0x3F80_0000;
        *(this.add(0x88) as *mut u32) = 0;
        core::ptr::write_unaligned(this.add(0x8C) as *mut u16, 0);
        *(this.add(0x98) as *mut u32) = 0xFFFF_FFFF;
        *this.add(0xA1) = 0;
        base
    }
});

