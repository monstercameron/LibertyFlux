// original: 0x00bdf340 audio_ctor_optional_ref
/// Construct the audio node with an optional referenced pointer.
/// Runs the shared base constructor (id 1), stamps vtable 0xEB9454,
/// zeroes +0x14/+0x18, sets +0x1c to -1 and the +0x20 flag byte to 0.
/// When the argument is non-null it is stored at +0x18 and registered
/// through the shared acquire helper (id 2). Returns the object.
export!(thiscall, rw_00bdf340(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB9454;
        callee_thiscall!(1, u32, this as u32);
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = 0;
        *((this.add(0x18)) as *mut u32) = 0;
        *((this.add(0x1C)) as *mut u32) = 0xFFFF_FFFF;
        *(this.add(0x20)) = 0;
        if arg != 0 {
            let slot = this.add(0x18) as *mut u32;
            *slot = arg;
            callee_stdcall!(2, u32, slot as u32);
        }
        this as u32
    }
});
