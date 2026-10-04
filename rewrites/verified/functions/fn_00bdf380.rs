// original: 0x00bdf380 audio_ctor_full_record
/// Construct the large audio record: two 12-byte vectors, scalar words,
/// a float, and an optional 16-byte tail block.
/// Runs the shared base constructor (id 1), stamps vtable 0xEB8D7C,
/// zeroes +0x20..+0x38, copies 12 bytes from each of the two vector
/// pointers, stores the scalar words and the float bit pattern, and zeroes
/// the +0x74 half-word. When the third scalar is non-null its slot
/// (object +0x64) is registered through the acquire helper (id 2). When
/// the tail pointer is non-null the ready flag at +0x75 is set and the
/// 16 tail bytes are copied over +0x20..+0x2c. Returns the object.
export!(thiscall, rw_00bdf380(
    this: *mut u8,
    a0: u32,
    a1: u32,
    a2: u32,
    fbits: u32,
    vec1: *const u32,
    vec2: *const u32,
    tail: *const u8,
) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8D7C;
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(VTABLE);
        // Six separate slots; +0x2c is deliberately not among them
        // (the tail block owns it when present).
        *((this.add(0x20)) as *mut u32) = 0;
        *((this.add(0x24)) as *mut u32) = 0;
        *((this.add(0x28)) as *mut u32) = 0;
        *((this.add(0x30)) as *mut u32) = 0;
        *((this.add(0x34)) as *mut u32) = 0;
        *((this.add(0x38)) as *mut u32) = 0;
        *((this.add(0x40)) as *mut u32) = *vec1;
        *((this.add(0x44)) as *mut u32) = *vec1.add(1);
        *((this.add(0x48)) as *mut u32) = *vec1.add(2);
        *((this.add(0x50)) as *mut u32) = *vec2;
        *((this.add(0x54)) as *mut u32) = *vec2.add(1);
        *((this.add(0x58)) as *mut u32) = *vec2.add(2);
        *((this.add(0x60)) as *mut u32) = 0;
        *((this.add(0x64)) as *mut u32) = a2;
        *((this.add(0x68)) as *mut u32) = a0;
        *((this.add(0x6C)) as *mut u32) = a1;
        *((this.add(0x70)) as *mut u32) = fbits;
        *((this.add(0x74)) as *mut u16) = 0;
        if a2 != 0 {
            callee_stdcall!(2, u32, this.add(0x64) as u32);
        }
        if !(tail as u32 == 0) {
            *(this.add(0x75)) = 1;
            *((this.add(0x20)) as *mut u32) = *(tail as *const u32);
            *((this.add(0x24)) as *mut u32) = *((tail.add(4)) as *const u32);
            *((this.add(0x28)) as *mut u32) = *((tail.add(8)) as *const u32);
            *((this.add(0x2C)) as *mut u32) = *((tail.add(0xC)) as *const u32);
        }
        this as u32
    }
});
