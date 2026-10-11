// original: 0x00B25190 CPhysical::vf59

/// Writes three zero words to the caller's output when the physics-object
/// lookup returns null. Otherwise copies three words from the returned
/// object's `+0x110`, `+0x114`, and `+0x118` fields to the output. The lookup
/// is a thiscall with no stack arguments; the output pointer is the method's
/// single stack argument.
lf_checker_rt::export!(thiscall, rw_b25190(this: u32, output: u32) -> () {
    const OUTPUT_WORDS: u32 = 3;
    const OBJECT_DATA: u32 = 0x110;
    const LOOKUP_SLOT: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let object = lf_checker_rt::callee_thiscall!(LOOKUP_SLOT, u32, this);
        if object == 0 {
            let mut word = 0;
            while word < OUTPUT_WORDS {
                write_u32(output.wrapping_add(word * 4), 0);
                word += 1;
            }
            return;
        }

        let mut word = 0;
        while word < OUTPUT_WORDS {
            let value = read_u32(object.wrapping_add(OBJECT_DATA + word * 4));
            write_u32(output.wrapping_add(word * 4), value);
            word += 1;
        }
    }
});
