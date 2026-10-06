// original: 0x00976320 audio_big_object_ctor (proposed)

/// Construct the large audio object: base constructor, vtable stamp, zeroed
/// header fields, then one member construction per sub-object slot.
/// Original: 0x00976320 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00976320(this: u32) -> u32 {
    unsafe {
        const VT: u32 = 0xE8BFC0;
        const BASE_CTOR: u32 = 1;
        const MEMBER: u32 = 2;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VT));
        ((this.wrapping_add(8)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0xC)) as *mut u16).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0xFA10));
        ((this.wrapping_add(0xFE50)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(0x10E60)) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10E70));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10E98));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10EC0));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10EE8));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10F10));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10F38));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10F60));
        lf_checker_rt::callee_thiscall!(MEMBER, u32, this.wrapping_add(0x10F88));
        this
    }
});
