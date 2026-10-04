// original: 0x00c2cc50 audFireAudioEntity::audFireAudioEntity
/// Constructor: base init, vtable stamp, member init, zero fields.
///
/// Runs the base initialiser, stamps the vtable pointer, initialises the
/// member at +0xC, zeroes four trailing fields and returns the object.
export!(thiscall, rw_00c2cc50(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(0xEC6DD8);
        callee_thiscall!(2, u32, (this as u32).wrapping_add(0xC));
        *(this.add(0x4C) as *mut u32) = 0;
        *(this.add(0x50) as *mut u32) = 0;
        *(this.add(0x54) as *mut u32) = 0;
        *(this.add(8) as *mut u32) = 0;
        this as u32
    }
});
