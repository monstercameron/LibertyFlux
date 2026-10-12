// original: 0x00A61BF0 record_key_equals

/// Compares the first key argument with the object's word at +0x30 and then,
/// only if that matches, compares the second argument with +0x34. The equality
/// tests are unsigned bitwise word comparisons. ECX is the object, two words
/// are passed on the thiscall stack, and AL returns the Boolean result.
lf_checker_rt::export!(thiscall, rw_00A61BF0(this: u32, first_key: u32, second_key: u32) -> u32 {
    const FIRST_KEY: u32 = 0x30;
    const SECOND_KEY: u32 = 0x34;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let first_matches = first_key == unsafe { read_u32(this.wrapping_add(FIRST_KEY)) };
    if !first_matches {
        return 0;
    }
    let second_matches = second_key == unsafe { read_u32(this.wrapping_add(SECOND_KEY)) };
    u32::from(second_matches)
});
