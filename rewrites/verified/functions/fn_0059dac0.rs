// original: 0x0059dac0 forward_object_slot
// Forward `(this + 0x1E6D6, 0xFE08C8, arg)` to a cdecl callee.
//
// The first argument is a field address derived from the object pointer;
// the checker normalizes heap-relative call arguments before comparing.
export!(thiscall, rw_0059DAC0(this: u32, a: u32) -> u32 {
    // The pushed constant is a relocated image address (HIGHLOW reloc on the
    // push immediate), so it must go through relocated(), not as a literal.
    callee_cdecl!(1, u32, this.wrapping_add(0x1E6D6), relocated(0xFE08C8), a)
});
