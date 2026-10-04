// original: 0x00bdf510 audio_ctor_vector_gain
/// Construct the audio node with a 16-byte vector, gain float and flag byte.
/// Runs the shared base constructor (id 1), stores the three scalar words
/// at +0x30/+0x34/+0x38, stamps vtable 0xEB91E4, copies the 16 vector bytes
/// to +0x20..+0x2c, stores the remaining words, the float bits at +0x44
/// and the flag byte at +0x3c. When the first scalar is non-null its slot
/// (object +0x30) is registered through the acquire helper (id 2) and the
/// ready flag at +0x4c is set, else the flag is cleared. Returns the object.
export!(thiscall, rw_00bdf510(
    this: *mut u8,
    a0: u32,
    a1: u32,
    a2: u32,
    vec: *const u32,
    a4: u32,
    fbits: u32,
    a6: u32,
    flag: u32,
) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB91E4;
        callee_thiscall!(1, u32, this as u32);
        *((this.add(0x30)) as *mut u32) = a0;
        *((this.add(0x34)) as *mut u32) = a1;
        *((this.add(0x38)) as *mut u32) = a2;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x20)) as *mut u32) = *vec;
        *((this.add(0x24)) as *mut u32) = *vec.add(1);
        *((this.add(0x28)) as *mut u32) = *vec.add(2);
        *((this.add(0x2C)) as *mut u32) = *vec.add(3);
        *((this.add(0x48)) as *mut u32) = a4;
        *((this.add(0x40)) as *mut u32) = a6;
        *((this.add(0x44)) as *mut u32) = fbits;
        *(this.add(0x3C)) = (flag & 0xFF) as u8;
        if a0 != 0 {
            callee_stdcall!(2, u32, this.add(0x30) as u32);
            *(this.add(0x4C)) = 1;
        } else {
            *(this.add(0x4C)) = 0;
        }
        this as u32
    }
});
