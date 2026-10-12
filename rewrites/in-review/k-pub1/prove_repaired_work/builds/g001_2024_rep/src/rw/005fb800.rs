// original: 0x005FB800 frag_instance_resolved_type

/// Resolve the referenced type at `+0x5c`: a null reference selects the
/// fallback pointer at `+0x6c`; a nonnull reference is accepted only when
/// its flags at `+8` contain both bits of mask `0x30000000`, in which case
/// the pointer at its first word is returned. Invalid flags return null.
/// This is a thiscall getter.

lf_checker_rt::export!(thiscall, rw_005fb800(this: u32) -> u32 {
    const REFERENCED_TYPE: u32 = 0x5c;
    const FALLBACK_OBJECT: u32 = 0x6c;
    const TYPE_FLAGS: u32 = 0x08;
    const TYPE_MASK: u32 = 0x3000_0000;

    unsafe {
        let referenced = read_u32(this.wrapping_add(REFERENCED_TYPE));
        if referenced == 0 {
            read_u32(this.wrapping_add(FALLBACK_OBJECT))
        } else if read_u32(referenced.wrapping_add(TYPE_FLAGS)) & TYPE_MASK == TYPE_MASK {
            read_u32(referenced)
        } else {
            0
        }
    }
});

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
