// original: 0x00ACB810 set_stream_item_flags_and_value

/// OR a flag mask into the object's streaming flags and replace its value.
///
/// `this + 0x164` holds the flag word and `this + 0x168` holds the float.
/// The method takes a mask and an `f32` on the stack and uses thiscall, with
/// the callee removing both stack arguments. The original leaves an incidental
/// integer in EAX, so the contract observes its memory effects and stack only.
lf_checker_rt::export!(thiscall, rw_00acb810(this: u32, mask: u32, value: f32) -> () {
    const FLAGS_OFFSET: u32 = 0x164;
    const VALUE_OFFSET: u32 = 0x168;

    unsafe {
        let flags = (this.wrapping_add(FLAGS_OFFSET)) as *mut u32;
        let value_slot = (this.wrapping_add(VALUE_OFFSET)) as *mut f32;
        flags.write_unaligned(flags.read_unaligned() | mask);
        value_slot.write_unaligned(value);
    }
});
