// original: 0x00cc7480 subobject_init_a
/// Initialise the first sub-object: stamp its two vtable slots, copy the
/// shared seed word from its global, and zero the state fields.
/// Returns the object pointer.
export!(thiscall, rw_00cc7480(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = relocated(0xED976C);
        *(this.add(4) as *mut u32) = *global::<u32>(0xFBBA3C);
        *(this.add(0x18C) as *mut u32) = 0;
        *this.add(0xC) = 0;
        *(this as *mut u32) = relocated(0xED992C);
        *(this.add(0x190) as *mut u32) = 0;
        this as u32
    }
});
