// original: 0x005d6ff0 html_element_stack_pop
/// If the low byte of the callee-cleaned stack flag is zero, replace the
/// process-global element-stack pointer with the pointer stored at offset
/// +8 in its current object and return that next pointer. For a nonzero low
/// byte, leave the global unchanged and preserve the incoming EAX value.
/// This function has one 32-bit stdcall argument but only reads its low byte;
/// the checker contract fixes entry EAX to the value preserved on that path.
lf_checker_rt::export!(stdcall, rw_005d6ff0(flag_word: u32) -> u32 {
    const ELEMENT_STACK_ADDRESS: u32 = 0x018b_6fac;
    const NEXT_POINTER_OFFSET: u32 = 0x08;
    const PRESERVED_ENTRY_EAX: u32 = 0xa5a5_a5a5;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    unsafe {
        if flag_word & 0xff != 0 {
            PRESERVED_ENTRY_EAX
        } else {
            let global = lf_checker_rt::global::<u32>(ELEMENT_STACK_ADDRESS);
            let current_object = global.read_unaligned();
            let next_pointer = read_u32(current_object.wrapping_add(NEXT_POINTER_OFFSET));
            global.write_unaligned(next_pointer);
            next_pointer
        }
    }
});
