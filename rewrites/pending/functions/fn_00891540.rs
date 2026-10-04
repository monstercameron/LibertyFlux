// original: 0x00891540 audio_set_fields_78_90
/// Store two parameter dwords into this object at +0x78 and +0x90.
///
/// Returns the second value, matching what the original leaves in EAX.
export!(thiscall, rw_00891540(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        ((this + 0x78) as *mut u32).write(a);
        ((this + 0x90) as *mut u32).write(b);
        b
    }
});
