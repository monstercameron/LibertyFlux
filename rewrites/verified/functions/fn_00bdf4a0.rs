// original: 0x00bdf4a0 audio_ctor_store2
/// Construct the audio node that stores two words at +0x18/+0x1c.
/// Runs the shared base constructor (id 1), stamps vtable 0xEB8E34,
/// zeroes the +0x14 slot and the +0x20 flag byte. Returns the object.
export!(thiscall, rw_00bdf4a0(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8E34;
        callee_thiscall!(1, u32, this as u32);
        *((this.add(0x18)) as *mut u32) = a;
        *((this.add(0x1C)) as *mut u32) = b;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = 0;
        *(this.add(0x20)) = 0;
        this as u32
    }
});
