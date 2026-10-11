// original: 0x005FB4E0 rage::fragInst::vf62

/// Read the unsigned 16-bit type index at instance offset `+0x08`, then call
/// the supplied context with that zero-extended index and two zero words.
/// The context is the method's one stack argument, the scripted helper's
/// return value is returned unchanged, and the thiscall method consumes its
/// one stack argument.
lf_checker_rt::export!(thiscall, rw_005fb4e0(this: u32, context: u32) -> u32 {
    const TYPE_INDEX: u32 = 0x08;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    unsafe {
        let index = u32::from(read_u16(this.wrapping_add(TYPE_INDEX)));
        lf_checker_rt::callee_thiscall!(1, u32, context, index, 0, 0)
    }
});
