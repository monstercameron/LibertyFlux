// original: 0x00bdf4d0 audio_ctor_store1_zero5
/// Construct the audio node that stores one word at +0x14.
/// Runs the shared base constructor (id 1), stamps vtable 0xEB8D24,
/// zeroes +0x18/+0x1c/+0x20 and the +0x24 half-word. Returns the object.
export!(thiscall, rw_00bdf4d0(this: *mut u8, a: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8D24;
        callee_thiscall!(1, u32, this as u32);
        *((this.add(0x14)) as *mut u32) = a;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x18)) as *mut u32) = 0;
        *((this.add(0x1C)) as *mut u32) = 0;
        *((this.add(0x20)) as *mut u32) = 0;
        *((this.add(0x24)) as *mut u16) = 0;
        this as u32
    }
});
