// original: 0x00bdf5c0 audio_ctor_default_plus_ref
/// Construct the audio node that chains the mixed-fields constructor.
/// Runs the mixed-fields constructor (id 1, thiscall/4) with fixed
/// arguments (0x3E8, 1, 0, 8.0f), stamps vtable 0xEB93A4, stores the
/// argument at +0x50 and, when non-null, registers that slot through
/// the acquire helper (id 2). Returns the object.
export!(thiscall, rw_00bdf5c0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB93A4;
        callee_thiscall!(1, u32, this as u32, 0x3E8u32, 1u32, 0u32, 0x4100_0000u32);
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x50)) as *mut u32) = arg;
        if arg != 0 {
            callee_stdcall!(2, u32, this.add(0x50) as u32);
        }
        this as u32
    }
});
