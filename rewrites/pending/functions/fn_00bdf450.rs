// original: 0x00bdf450 audio_ctor_mixed_fields
/// Construct the audio node with a word, two flag bytes and a float.
/// Runs the shared base constructor (id 1), stores arg0 at +0x30, the
/// float arg at +0x34, stamps vtable 0xEB8CCC, zeroes +0x38/+0x3c and the
/// +0x40 half-word, and stores the two low bytes at +0x44/+0x45.
/// Returns the object.
export!(thiscall, rw_00bdf450(this: *mut u8, a: u32, b: u32, c: u32, fbits: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8CCC;
        callee_thiscall!(1, u32, this as u32);
        *((this.add(0x30)) as *mut u32) = a;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x34)) as *mut u32) = fbits;
        *((this.add(0x38)) as *mut u32) = 0;
        *((this.add(0x3C)) as *mut u32) = 0;
        *((this.add(0x40)) as *mut u16) = 0;
        *(this.add(0x44)) = (b & 0xFF) as u8;
        *(this.add(0x45)) = (c & 0xFF) as u8;
        this as u32
    }
});
